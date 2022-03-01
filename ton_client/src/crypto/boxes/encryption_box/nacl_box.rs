use std::sync::Arc;

use zeroize::ZeroizeOnDrop;

use crate::ClientContext;
use crate::crypto::{EncryptionBox, EncryptionBoxInfo, nacl_box, nacl_box_open, ParamsOfNaclBox, ParamsOfNaclBoxOpen};
use crate::error::ClientResult;

#[derive(Serialize, Deserialize, Clone, Debug, ApiType, Default, PartialEq, ZeroizeOnDrop)]
pub struct NaclBoxParams {
    /// 256-bit key. Must be encoded with `hex`.
    pub their_public: String,
    /// 256-bit key. Must be encoded with `hex`.
    pub secret: String,
    /// 96-bit nonce. Must be encoded with `hex`.
    pub nonce: String,
}

#[derive(Debug)]
pub(crate) struct NaclEncryptionBox {
    params: NaclBoxParams,
    hdpath: Option<String>,
}

impl NaclEncryptionBox {
    pub fn new(params: NaclBoxParams, hdpath: Option<String>) -> Self {
        Self { params, hdpath }
    }
}

#[async_trait::async_trait]
impl EncryptionBox for NaclEncryptionBox {
    async fn get_info(&self, _context: Arc<ClientContext>) -> ClientResult<EncryptionBoxInfo> {
        Ok(EncryptionBoxInfo {
            algorithm: Some("NaclBox".to_owned()),
            hdpath: self.hdpath.clone(),
            public: None,
            options: Some(json!({
                "their_public": &self.params.their_public,
                "nonce": hex::encode(&self.params.nonce),
            }))
        })
    }

    async fn encrypt(&self, context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        nacl_box(context, ParamsOfNaclBox {
            decrypted: data.clone(),
            nonce: self.params.nonce.clone(),
            their_public: self.params.their_public.clone(),
            secret: self.params.secret.clone(),
        }).map(|result| result.encrypted)
    }

    async fn decrypt(&self, context: Arc<ClientContext>, data: &String) -> ClientResult<String> {
        nacl_box_open(context, ParamsOfNaclBoxOpen {
            encrypted: data.clone(),
            nonce: self.params.nonce.clone(),
            their_public: self.params.their_public.clone(),
            secret: self.params.secret.clone(),
        }).map(|result| result.decrypted)
    }
}
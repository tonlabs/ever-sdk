use crate::abi::{Error, FunctionHeader};
use crate::boc::internal::deserialize_object_from_boc;
use crate::client::ClientContext;
use crate::error::ClientResult;
use crate::{abi::types::Abi, boc::internal::deserialize_cell_from_boc};
use serde_json::Value;
use std::sync::Arc;
use ton_abi::contract::DecodedMessage;
use ton_abi::token::Detokenizer;
use ton_abi::{Function, TokenValue};
use ton_block::MsgAddressInt;
use ton_sdk::AbiContract;
use ton_types::SliceData;

#[derive(Serialize, Deserialize, ApiType, PartialEq, Debug, Clone)]
pub enum MessageBodyType {
    /// Message contains the input of the ABI function.
    Input,

    /// Message contains the output of the ABI function.
    Output,

    /// Message contains the input of the imported ABI function.
    ///
    /// Occurs when contract sends an internal message to other
    /// contract.
    InternalOutput,

    /// Message contains the input of the ABI event.
    Event,
}

impl MessageBodyType {
    pub fn is_output(&self) -> bool {
        match self {
            MessageBodyType::InternalOutput | MessageBodyType::Output => true,
            _ => false,
        }
    }
}

pub struct ResponsibleCall<'a> {
    pub function: &'a Function,
    pub src: &'a MsgAddressInt,
    pub answer_id: u32,
}

#[derive(Serialize, Deserialize, ApiType, PartialEq, Debug, Clone)]
pub struct DecodedMessageBody {
    /// Type of the message body content.
    pub body_type: MessageBodyType,

    /// Function or event name.
    pub name: String,

    /// Parameters or result value.
    pub value: Option<Value>,

    /// Function header.
    pub header: Option<FunctionHeader>,
}

impl DecodedMessageBody {
    fn new(
        body_type: MessageBodyType,
        decoded: DecodedMessage,
        header: Option<FunctionHeader>,
    ) -> ClientResult<Self> {
        let value = Detokenizer::detokenize_to_json_value(&decoded.tokens)
            .map_err(|x| Error::invalid_message_for_decode(x))?;
        Ok(Self {
            body_type,
            name: decoded.function_name,
            value: Some(value),
            header,
        })
    }

    pub(crate) fn decode(
        abi: &AbiContract,
        responsible: Option<&ResponsibleCall>,
        body: SliceData,
        is_internal: bool,
        internal_dst: Option<&MsgAddressInt>,
        allow_partial: bool,
    ) -> ClientResult<Self> {
        match (internal_dst, responsible) {
            (Some(internal_dst), Some(ref responsible))
                if is_internal && internal_dst == responsible.src =>
            {
                let mut body = body.clone();

                let receiver_func_id = body.get_next_u32().map_err(|err| {
                    Error::invalid_message_for_decode(format!(
                        "Can't decode function header: {}",
                        err
                    ))
                })?;

                if receiver_func_id == responsible.answer_id {
                    let tokens = TokenValue::decode_params(
                        responsible.function.output_params(),
                        body,
                        &abi.version(),
                        allow_partial,
                    )
                    .map_err(|err| {
                        Error::invalid_message_for_decode(format!(
                            "Responsible function output can't be decoded: {}",
                            err
                        ))
                    })?;

                    let decoded = DecodedMessage {
                        function_name: responsible.function.name.clone(),
                        tokens,
                    };
                    return DecodedMessageBody::new(MessageBodyType::InternalOutput, decoded, None);
                }
            }
            _ => {}
        }
        if let Ok(output) = abi.decode_output(body.clone(), is_internal, allow_partial) {
            if abi.events().get(&output.function_name).is_some() {
                DecodedMessageBody::new(MessageBodyType::Event, output, None)
            } else {
                DecodedMessageBody::new(MessageBodyType::Output, output, None)
            }
        } else if let Ok(input) = abi.decode_input(body.clone(), is_internal, allow_partial) {
            let (header, _, _) = ton_abi::Function::decode_header(
                abi.version(),
                body.clone(),
                abi.header(),
                is_internal,
            )
            .map_err(|err| {
                Error::invalid_message_for_decode(format!("Can't decode function header: {}", err))
            })?;
            DecodedMessageBody::new(
                MessageBodyType::Input,
                input,
                FunctionHeader::from(&header)?,
            )
        } else {
            Err(Error::invalid_message_for_decode(
                "The message body does not match the specified ABI.\n
                Tip: Please check that you specified message's body, not full BOC.",
            ))
        }
    }

    pub async fn decode_message<'a>(
        context: Arc<ClientContext>,
        params: ParamsOfDecodeMessage,
        responsible: Option<&ResponsibleCall<'a>>,
    ) -> ClientResult<DecodedMessageBody> {
        let (abi, message) = prepare_decode(&context, &params).await?;
        if let Some(body) = message.body() {
            Self::decode(
                &abi,
                responsible,
                body,
                message.is_internal(),
                message.dst_ref(),
                params.allow_partial,
            )
        } else {
            Err(Error::invalid_message_for_decode(
                "The message body is empty",
            ))
        }
    }
}

//---------------------------------------------------------------------------------- decode_message

#[derive(Serialize, Deserialize, ApiType, Default)]
pub struct ParamsOfDecodeMessage {
    /// contract ABI
    pub abi: Abi,

    /// Message BOC
    pub message: String,

    /// Flag allowing partial BOC decoding when ABI doesn't describe the full body BOC.
    /// Controls decoder behaviour when after decoding all described in ABI params there are some data left in BOC:
    /// `true` - return decoded values
    /// `false` - return error of incomplete BOC deserialization (default)
    #[serde(default)]
    pub allow_partial: bool,
}

/// Decodes message body using provided message BOC and ABI.
#[api_function]
pub async fn decode_message(
    context: Arc<ClientContext>,
    params: ParamsOfDecodeMessage,
) -> ClientResult<DecodedMessageBody> {
    let (abi, message) = prepare_decode(&context, &params).await?;
    if let Some(body) = message.body() {
        DecodedMessageBody::decode(
            &abi,
            None,
            body,
            message.is_internal(),
            message.dst_ref(),
            params.allow_partial,
        )
    } else {
        Err(Error::invalid_message_for_decode(
            "The message body is empty",
        ))
    }
}

//----------------------------------------------------------------------------- decode_message_body

#[derive(Serialize, Deserialize, ApiType, Default)]
pub struct ParamsOfDecodeMessageBody {
    /// Contract ABI used to decode.
    pub abi: Abi,

    /// Message body BOC encoded in `base64`.
    pub body: String,

    /// True if the body belongs to the internal message.
    pub is_internal: bool,

    /// Flag allowing partial BOC decoding when ABI doesn't describe the full body BOC.
    /// Controls decoder behaviour when after decoding all described in ABI params there are some data left in BOC:
    /// `true` - return decoded values
    /// `false` - return error of incomplete BOC deserialization (default)
    #[serde(default)]
    pub allow_partial: bool,
}

/// Decodes message body using provided body BOC and ABI.
#[api_function]
pub async fn decode_message_body(
    context: Arc<ClientContext>,
    params: ParamsOfDecodeMessageBody,
) -> ClientResult<DecodedMessageBody> {
    let abi = params.abi.json_string()?;
    let abi = AbiContract::load(abi.as_bytes()).map_err(|x| Error::invalid_json(x))?;
    let (_, body) = deserialize_cell_from_boc(&context, &params.body, "message body").await?;
    DecodedMessageBody::decode(
        &abi,
        None,
        body.into(),
        params.is_internal,
        None,
        params.allow_partial,
    )
}

async fn prepare_decode(
    context: &ClientContext,
    params: &ParamsOfDecodeMessage,
) -> ClientResult<(AbiContract, ton_block::Message)> {
    let abi = params.abi.json_string()?;
    let abi = AbiContract::load(abi.as_bytes()).map_err(|x| Error::invalid_json(x))?;
    let message = deserialize_object_from_boc(context, &params.message, "message")
        .await
        .map_err(|x| Error::invalid_message_for_decode(x))?;
    Ok((abi, message.object))
}

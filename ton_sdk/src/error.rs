/*
* Copyright 2018-2021 TON DEV SOLUTIONS LTD.
*
* Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
* this file except in compliance with the License.
*
* Unless required by applicable law or agreed to in writing, software
* distributed under the License is distributed on an "AS IS" BASIS,
* WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
* See the License for the specific TON DEV software governing permissions and
* limitations under the License.
*/

#[derive(Debug, failure::Fail)]
pub enum SdkError {
    #[fail(display = "Invalid data: {}", msg)]
    InvalidData { msg: String },

    #[fail(display = "Internal error: {}", msg)]
    InternalError { msg: String },
}

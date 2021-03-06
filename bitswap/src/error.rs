use futures::channel::{mpsc, oneshot};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BitswapError {
    #[error("Error while decoding bitswap message: {0}")]
    ProtobufError(#[from] prost::DecodeError),
    #[error("Error while parsing cid: {0}")]
    Cid(#[from] cid::Error),
    #[error("Invalid Message")]
    InvalidData,
    #[error("Closing")]
    Closing,
    #[error("Timeout")]
    Timeout,
    #[error("Error sending {0}")]
    Send(#[from] mpsc::SendError),
    #[error("Cancelled oneshot {0}")]
    Cancel(#[from] oneshot::Canceled),
}
//
// impl From<mpsc::SendError> for BitswapError {
//     fn from(_: mpsc::SendError) -> Self {
//         BitswapError::Send
//     }
// }

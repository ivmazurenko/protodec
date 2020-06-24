use crate::error::ProtodecError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UiMessage {
    OpenFile, // not used
    InitialStateInputChanged(String),
    ProcessByteArray,
    FileLoaded(Result<Vec<u8>, ProtodecError>),
    DecodeChunkAsMessage(Uuid),
    DecodeChunkAsUtf8String(Uuid),
    Toggle(Uuid),
}

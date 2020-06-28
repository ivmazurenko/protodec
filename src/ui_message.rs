use crate::web_sys::FileList;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UiMessage {
    InitialStateDragEnter,
    InitialStateDragOver,
    InitialStateDragLeave,
    InitialStateDrop(FileList),
    InitialStateFileRead { file_name: String, buffer: Vec<u8> },
    //
    InitialStateInputChanged(String),
    ProcessByteArray,
    ProcessUploadedFile,
    DecodeChunkAsMessage(Uuid),
    DecodeChunkAsUtf8String(Uuid),
    Toggle(Uuid),
}

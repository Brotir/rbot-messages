use crate::{messages::messages::Message, MessageIdentity};
use serde::Serialize;
use serde_json;

pub fn serialize_message<T: Message + Serialize + MessageIdentity>(
    message: &T,
) -> Result<Vec<u8>, std::io::Error> {
    // ---- Message parts ----
    let mut bytes = T::id().to_le_bytes().to_vec();
    let serialized_message = serde_json::to_vec(message)?;
    let message_size = (serialized_message.len() as i32).to_le_bytes();

    // ----  Combine into full message ----
    // [--TYPE--, --SIZE--, --BYTES--] : [u8]
    bytes.extend(message_size);
    bytes.extend(serialized_message);

    Ok(bytes)
}

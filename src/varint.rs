pub fn take_varint(buffer: &[u8]) -> (u128, &[u8]) {
    let mut shifter = 0;
    let mut value: u128 = 0;

    let mut index_to_skip = 0;
    for (index, byte) in buffer.iter().enumerate() {
        index_to_skip = index;
        value |= (*byte as u128 & 0x7F) << shifter;
        shifter += 7;

        if (byte & 0x80) == 0 {
            break;
        }
    }

    (value, &buffer[index_to_skip + 1..])
}

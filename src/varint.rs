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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_takes_varint_300() {
        let buffer = [0b_1010_1100, 0b_0000_0010];

        let actual = take_varint(&buffer);

        assert_eq!(300, actual.0);
        assert_eq!(0, actual.1.len());
    }

    #[test]
    fn correctly_takes_varint_1() {
        let buffer = [0b_0001];

        let actual = take_varint(&buffer);

        assert_eq!(1, actual.0);
        assert_eq!(0, actual.1.len());
    }

    #[test]
    fn returns_correct_remaining_tail() {
        let buffer = [0b_0001, 0, 0, 0, 0];

        let actual = take_varint(&buffer);

        assert_eq!(4, actual.1.len());
    }

    #[test]
    fn returns_correct_remaining_tail_2() {
        let buffer = [0b_0001, 255, 128, 64, 32];

        let actual = take_varint(&buffer);

        assert_eq!(4, actual.1.len());
    }
}

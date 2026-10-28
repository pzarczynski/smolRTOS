#![no_std]

/// Returns the highest ready priority.
/// Higher numbers mean higher priorities.
pub fn highest_ready(bitmap: u32) -> Option<u8> {
    if bitmap == 0 {
        None
    } else {
        Some((31 - bitmap.leading_zeros()) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::highest_ready;

    #[test]
    fn empty_ready_set() {
        assert_eq!(highest_ready(0), None);
    }

    #[test]
    fn selects_highest_priority() {
        let bitmap = (1 << 2) | (1 << 5) | (1 << 9);
        assert_eq!(highest_ready(bitmap), Some(9));
    }

    #[test]
    fn supports_highest_bit() {
        assert_eq!(highest_ready(1 << 31), Some(31));
    }
}
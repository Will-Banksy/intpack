// Byte numbering: Index 0 in input slice is the most significant byte(s) in the output value

// Note on casting unsigned numbers with as: Casting to smaller integer will truncate, casting to larger will zero-extend

// Functions for every unsigned type to pack multiple of them into a bigger type

use std::convert::TryInto;

// This simply panics on fail. It shouldn't ever fail
macro_rules! to_arr {
	($slice:expr) => {
		$slice.try_into().unwrap()
	};
}

// Base packing functions to pack to next biggest

/// Packs 2 u8 values into a u16. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u8_to_u16(b: &[u8; 2]) -> u16 {
	((b[0] as u16) << 8)
	| (b[1] as u16)
}

/// Packs 2 u16 values into a u32. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u16_to_u32(b: &[u16; 2]) -> u32 {
	((b[0] as u32) << 16)
	| (b[1] as u32)
}

/// Packs 2 u32 values into a u64. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u32_to_u64(b: &[u32; 2]) -> u64 {
	((b[0] as u64) << 16)
	| (b[1] as u64)
}

/// Packs 2 u64 values into a u128. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u64_to_u128(b: &[u64; 2]) -> u128 {
	((b[0] as u128) << 64)
	| (b[1] as u128)
}

// And then extended packing functions, pack into everything bigger

/// Packs 4 u8 values into a u32. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u8_to_u32(b: &[u8; 4]) -> u32 {
	u16_to_u32(&[u8_to_u16(to_arr!(&b[0..=1])), u8_to_u16(to_arr!(&b[2..=3]))])
}

/// Packs 8 u8 values into a u64. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u8_to_u64(b: &[u8; 8]) -> u64 {
	u32_to_u64(&[u8_to_u32(to_arr!(&b[0..=3])), u8_to_u32(to_arr!(&b[4..=8]))])
}

/// Packs 16 u8 values into a u128. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u8_to_u128(b: &[u8; 16]) -> u128 {
	u64_to_u128(&[u8_to_u64(to_arr!(&b[0..=7])), u8_to_u64(to_arr!(&b[8..=15]))])
}

/// Packs 4 u16 values into a u64. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u16_to_u64(b: &[u16; 4]) -> u64 {
	u32_to_u64(&[u16_to_u32(to_arr!(&b[0..=1])), u16_to_u32(to_arr!(&b[2..=3]))])
}

/// Packs 4 u16 values into a u128. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u16_to_u128(b: &[u16; 8]) -> u128 {
	u64_to_u128(&[u16_to_u64(to_arr!(&b[0..=3])), u16_to_u64(to_arr!(&b[4..=8]))])
}

/// Packs 4 u32 values into a u128. Index 0 in input slice is the most significant byte(s) in the output value
pub fn u32_to_u128(b: &[u32; 4]) -> u128 {
	u64_to_u128(&[u32_to_u64(to_arr!(&b[0..=1])), u32_to_u64(to_arr!(&b[2..=3]))])
}

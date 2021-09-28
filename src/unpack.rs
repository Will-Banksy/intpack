// Byte numbering: Most significant bytes in input value is at index 0 in output array

// Note on casting unsigned numbers with as: Casting to smaller integer will truncate, casting to larger will zero-extend

// Functions for every unsigned type to unpack them into multiple smaller types

use std::convert::TryInto;

// Base unpacking functions to unpack to next smallest

pub fn u128_to_u64(b: u128) -> [u64; 2] {
	[
		(b >> 64) as u64,
		b as u64
	]
}

pub fn u64_to_u32(b: u64) -> [u32; 2] {
	[
		(b >> 32) as u32,
		b as u32
	]
}

pub fn u32_to_u16(b: u32) -> [u16; 2] {
	[
		(b >> 16) as u16,
		b as u16
	]
}

pub fn u16_to_u8(b: u16) -> [u8; 2] {
	[
		(b >> 8) as u8,
		b as u8
	]
}

// And then extended unpacking functions, unpack into everything smaller

pub fn u128_to_u32(b: u128) -> [u32; 4] {
	let inter_arr = u128_to_u64(b);
	[
		u64_to_u32(inter_arr[1]),
		u64_to_u32(inter_arr[0])
	].concat().try_into().unwrap()
}

pub fn u128_to_u16(b: u128) -> [u16; 8] {
	let inter_arr = u128_to_u32(b);
	[
		u32_to_u16(inter_arr[3]),
		u32_to_u16(inter_arr[2]),
		u32_to_u16(inter_arr[1]),
		u32_to_u16(inter_arr[0])
	].concat().try_into().unwrap()
}

pub fn u128_to_u8(b: u128) -> [u8; 16] {
	let inter_arr = u128_to_u16(b);
	[
		u16_to_u8(inter_arr[7]),
		u16_to_u8(inter_arr[6]),
		u16_to_u8(inter_arr[5]),
		u16_to_u8(inter_arr[4]),
		u16_to_u8(inter_arr[3]),
		u16_to_u8(inter_arr[2]),
		u16_to_u8(inter_arr[1]),
		u16_to_u8(inter_arr[0])
	].concat().try_into().unwrap()
}

pub fn u64_to_u16(b: u64) -> [u16; 4] {
	let inter_arr = u64_to_u32(b);
	[
		u32_to_u16(inter_arr[1]),
		u32_to_u16(inter_arr[0])
	].concat().try_into().unwrap()
}

pub fn u64_to_u8(b: u64) -> [u8; 8] {
	let inter_arr = u64_to_u16(b);
	[
		u16_to_u8(inter_arr[3]),
		u16_to_u8(inter_arr[2]),
		u16_to_u8(inter_arr[1]),
		u16_to_u8(inter_arr[0])
	].concat().try_into().unwrap()
}

pub fn u32_to_u8(b: u32) -> [u8; 4] {
	let inter_arr = u32_to_u16(b);
	[
		u16_to_u8(inter_arr[1]),
		u16_to_u8(inter_arr[0])
	].concat().try_into().unwrap()
}

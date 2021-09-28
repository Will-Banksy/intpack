fn main() {
	use intpack::pack;

	let result = pack::u8_to_u32(&[0xff, 0x00, 0xff, 0x00]);
	println!("Result: {}", result);

	use intpack::unpack;

	let result = unpack::u32_to_u8(0xff00ff00);
	println!("Result: [0]:{}, [1]:{}, [2]:{}, [3]:{}", result[0], result[1], result[2], result[3]);
}
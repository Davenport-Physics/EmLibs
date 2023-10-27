use rand::Rng;

use uuid::{Uuid};

pub fn get_random_string32() -> String {

	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
							abcdefghijklmnopqrstuvwxyz";
	const RAND_STRING_LEN: usize = 32;
	let mut rng = rand::thread_rng();

	let rand_string: String = (0..RAND_STRING_LEN)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect();

	rand_string
}

pub fn get_random_string(length: i32) -> String {

	const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
							abcdefghijklmnopqrstuvwxyz";
	let mut rng = rand::thread_rng();

	let rand_string: String = (0..length)
		.map(|_| {
			let idx = rng.gen_range(0..CHARSET.len());
			CHARSET[idx] as char
		})
		.collect();

	rand_string
}

pub fn get_uuid() -> String {

	Uuid::new_v4().hyphenated().to_string()

}

pub fn get_uuid_from_str(uuid_str: &str) -> Uuid {

	Uuid::parse_str(uuid_str).unwrap()

}
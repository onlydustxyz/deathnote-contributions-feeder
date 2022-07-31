use super::*;
use serde_test::{assert_de_tokens, assert_de_tokens_error, Token};

#[test]
fn test_de_hex_prefixed_string() {
	let test_string = HexPrefixedString("0x0123456789abcdef".to_string());

	assert_de_tokens(&test_string, &[Token::Str("0x0123456789abcdef")])
}

#[test]
fn test_de_hex_prefixed_string_uppercase() {
	let result_string = HexPrefixedString("0x0123456789abcdef".to_string());

	assert_de_tokens(&result_string, &[Token::Str("0x0123456789ABCDEF")])
}

#[test]
fn test_de_hex_prefixed_string_0x0() {
	let test_string = HexPrefixedString("0x0".to_string());

	assert_de_tokens(&test_string, &[Token::Str("0x0")])
}

#[test]
fn test_de_hex_prefixed_string_wrong_prefix() {
	let test_str = "xxabc";
	assert_de_tokens_error::<HexPrefixedString>(
		&[Token::Str(test_str)],
		&format!(
			"invalid value: string \"{}\", expected a \"0x\" prefixed string representation of an hexadecimal number",
			test_str
		),
	)
}

#[test]
fn test_de_hex_prefixed_string_too_short() {
	let test_str = "0x";
	assert_de_tokens_error::<HexPrefixedString>(
		&[Token::Str(test_str)],
		&format!(
			"invalid length {}, expected a \"0x\" prefixed string representation of an hexadecimal number",
			test_str.len()
		),
	)
}

#[test]
fn test_de_hex_prefixed_string_non_af_char() {
	let test_str = "0x0t";
	assert_de_tokens_error::<HexPrefixedString>(
		&[Token::Str(test_str)],
		&format!(
			"invalid value: string \"{}\", expected a \"0x\" prefixed string representation of an hexadecimal number",
			test_str
		),
	)
}

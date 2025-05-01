use crate::lexer::tokenizer::{Token, TokenType};
use crate::lexer::utils::{cmp_char, is_hex_digit};

const PLUS_SIGN: u8 = b'+';
const HYPHEN_MINUS: u8 = b'-';
const QUESTION_MARK: u8 = b'?';
const U: u8 = b'u';

fn is_delim(token: &Token, code: u8) -> bool {
    matches!(token.token_type, TokenType::Delim) && token.value.as_bytes()[0] == code
}

fn starts_with(token: &Token, code: u8) -> bool {
    token.value.as_bytes()[0] == code
}

fn hex_sequence(token: &Token, offset: usize, allow_dash: bool) -> usize {
    let mut hex_len = 0;
    let bytes = token.value.as_bytes();

    for pos in offset..bytes.len() {
        let code = bytes[pos];

        if code == HYPHEN_MINUS && allow_dash && hex_len != 0 {
            // Allow a dash only if it's not the first character
            return hex_len + 1; // Disallow following question marks
        }

        if !is_hex_digit(code) {
            return 0; // Not a hex digit
        }

        hex_len += 1;

        if hex_len > 6 {
            return 0; // Too many hex digits
        }
    }

    hex_len
}

fn with_question_mark_sequence<F>(
    mut consumed: usize,
    mut length: usize,
    get_next_token: F,
) -> usize
where
    F: Fn(usize) -> Option<Token>,
{
    if consumed == 0 {
        return 0; // Nothing consumed
    }

    while let Some(token) = get_next_token(length) {
        if !is_delim(&token, QUESTION_MARK) {
            break;
        }

        consumed += 1;
        if consumed > 6 {
            return 0; // Too many question marks
        }

        length += 1;
    }

    length
}

// https://drafts.csswg.org/css-syntax/#urange
// Informally, the <urange> production has three forms:
// U+0001
//      Defines a range consisting of a single code point, in this case the code point "1".
// U+0001-00ff
//      Defines a range of codepoints between the first and the second value, in this case
//      the range between "1" and "ff" (255 in decimal) inclusive.
// U+00??
//      Defines a range of codepoints where the "?" characters range over all hex digits,
//      in this case defining the same as the value U+0000-00ff.
// In each form, a maximum of 6 digits is allowed for each hexadecimal number (if you treat "?" as a hexadecimal digit).
//
// <urange> =
//   u '+' <ident-token> '?'* |
//   u <dimension-token> '?'* |
//   u <number-token> '?'* |
//   u <number-token> <dimension-token> |
//   u <number-token> <number-token> |
//   u '+' '?'+
pub fn urange<F>(mut token: Option<Token>, get_next_token: F) -> usize
where
    F: Fn(usize) -> Option<Token>,
{
    let mut length = 0;

    // Should start with `u` or `U`
    if token.is_none()
        || token.as_ref().unwrap().token_type != TokenType::Ident
        || !cmp_char(&token.as_ref().unwrap().value, 0, U)
    {
        return 0;
    }

    token = get_next_token(length + 1);
    length += 1;

    if token.is_none() {
        return 0;
    }

    let token = token.unwrap();

    // u '+' <ident-token> '?'*
    // u '+' '?'+
    if is_delim(&token, PLUS_SIGN) {
        let mut next_token = get_next_token(length + 1);
        length += 1;

        if next_token.is_none() {
            return 0;
        }

        let next_token = next_token.unwrap();

        if next_token.token_type == TokenType::Ident {
            // u '+' <ident-token> '?'*
            return with_question_mark_sequence(
                hex_sequence(&next_token, 0, true),
                length + 1,
                get_next_token,
            );
        }

        if is_delim(&next_token, QUESTION_MARK) {
            // u '+' '?'+
            return with_question_mark_sequence(1, length + 1, get_next_token);
        }

        // Hex digit or question mark is expected
        return 0;
    }

    // u <number-token> '?'*
    // u <number-token> <dimension-token>
    // u <number-token> <number-token>
    if token.token_type == TokenType::Number {
        let consumed_hex_length = hex_sequence(&token, 1, true);
        if consumed_hex_length == 0 {
            return 0;
        }

        let mut next_token = get_next_token(length + 1);
        length += 1;

        if next_token.is_none() {
            // u <number-token> <eof>
            return length;
        }

        let next_token = next_token.unwrap();

        if next_token.token_type == TokenType::Dimension
            || next_token.token_type == TokenType::Number
        {
            // u <number-token> <dimension-token>
            // u <number-token> <number-token>
            if !starts_with(&next_token, HYPHEN_MINUS) || hex_sequence(&next_token, 1, false) == 0 {
                return 0;
            }

            return length + 1;
        }

        // u <number-token> '?'*
        return with_question_mark_sequence(consumed_hex_length, length, get_next_token);
    }

    // u <dimension-token> '?'*
    if token.token_type == TokenType::Dimension {
        return with_question_mark_sequence(
            hex_sequence(&token, 1, true),
            length + 1,
            get_next_token,
        );
    }

    0
}

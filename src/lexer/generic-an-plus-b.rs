use crate::lexer::tokenizer::{Token, TokenType};
use crate::lexer::utils::{cmp_char, is_digit};

const PLUS_SIGN: u8 = b'+';
const HYPHEN_MINUS: u8 = b'-';
const N: u8 = b'n';
const DISALLOW_SIGN: bool = true;
const ALLOW_SIGN: bool = false;

fn is_delim(token: &Token, code: u8) -> bool {
    matches!(token.token_type, TokenType::Delim) && token.value.as_bytes()[0] == code
}

fn skip_sc<F>(mut token: Token, mut offset: usize, get_next_token: F) -> usize
where
    F: Fn(usize) -> Token,
{
    while matches!(token.token_type, TokenType::WhiteSpace | TokenType::Comment) {
        offset += 1;
        token = get_next_token(offset);
    }
    offset
}

fn check_integer<F>(
    token: &Token,
    mut value_offset: usize,
    disallow_sign: bool,
    offset: usize,
) -> usize
where
    F: Fn(usize) -> Token,
{
    if token.value.is_empty() {
        return 0;
    }

    let bytes = token.value.as_bytes();
    if bytes[value_offset] == PLUS_SIGN || bytes[value_offset] == HYPHEN_MINUS {
        if disallow_sign {
            return 0; // Number sign is not allowed
        }
        value_offset += 1;
    }

    while value_offset < bytes.len() {
        if !is_digit(bytes[value_offset]) {
            return 0; // Integer is expected
        }
        value_offset += 1;
    }

    offset + 1
}

fn consume_b<F>(mut token: Token, offset_: usize, get_next_token: F) -> usize
where
    F: Fn(usize) -> Token,
{
    let mut sign = false;
    let mut offset = skip_sc(token.clone(), offset_, &get_next_token);

    token = get_next_token(offset);

    if token.token_type != TokenType::Number {
        if is_delim(&token, PLUS_SIGN) || is_delim(&token, HYPHEN_MINUS) {
            sign = true;
            offset = skip_sc(get_next_token(offset + 1), offset, &get_next_token);
            token = get_next_token(offset);

            if token.token_type != TokenType::Number {
                return 0;
            }
        } else {
            return offset_;
        }
    }

    if !sign {
        let code = token.value.as_bytes()[0];
        if code != PLUS_SIGN && code != HYPHEN_MINUS {
            return 0; // Number sign is expected
        }
    }

    check_integer::<F>(&token, if sign { 0 } else { 1 }, sign, offset)
}

pub fn an_plus_b<F>(mut token: Token, get_next_token: F) -> usize
where
    F: Fn(usize) -> Token,
{
    let mut offset = 0;

    if token.token_type == TokenType::Number {
        return check_integer::<F>(&token, 0, ALLOW_SIGN, offset); // b
    }

    if token.token_type == TokenType::Ident && token.value.as_bytes()[0] == HYPHEN_MINUS {
        if !cmp_char(&token.value, 1, N) {
            return 0;
        }

        match token.value.len() {
            2 => return consume_b(get_next_token(offset + 1), offset + 1, &get_next_token),
            3 if token.value.as_bytes()[2] == HYPHEN_MINUS => {
                offset = skip_sc(get_next_token(offset + 1), offset, &get_next_token);
                token = get_next_token(offset);
                return check_integer::<F>(&token, 0, DISALLOW_SIGN, offset);
            }
            _ if token.value.as_bytes()[2] == HYPHEN_MINUS => {
                return check_integer::<F>(&token, 3, DISALLOW_SIGN, offset);
            }
            _ => return 0,
        }
    }

    if token.token_type == TokenType::Ident
        || (is_delim(&token, PLUS_SIGN)
            && get_next_token(offset + 1).token_type == TokenType::Ident)
    {
        if token.token_type != TokenType::Ident {
            token = get_next_token(offset + 1);
        }

        if !cmp_char(&token.value, 0, N) {
            return 0;
        }

        match token.value.len() {
            1 => return consume_b(get_next_token(offset + 1), offset + 1, &get_next_token),
            2 if token.value.as_bytes()[1] == HYPHEN_MINUS => {
                offset = skip_sc(get_next_token(offset + 1), offset, &get_next_token);
                token = get_next_token(offset);
                return check_integer::<F>(&token, 0, DISALLOW_SIGN, offset);
            }
            _ if token.value.as_bytes()[1] == HYPHEN_MINUS => {
                return check_integer::<F>(&token, 2, DISALLOW_SIGN, offset);
            }
            _ => return 0,
        }
    }

    if token.token_type == TokenType::Dimension {
        let bytes = token.value.as_bytes();
        let mut i = if bytes[0] == PLUS_SIGN || bytes[0] == HYPHEN_MINUS {
            1
        } else {
            0
        };

        while i < bytes.len() && is_digit(bytes[i]) {
            i += 1;
        }

        if i == 0 || !cmp_char(&token.value, i, N) {
            return 0;
        }

        if i + 1 == bytes.len() {
            return consume_b(get_next_token(offset + 1), offset + 1, &get_next_token);
        } else if bytes[i + 1] == HYPHEN_MINUS {
            if i + 2 == bytes.len() {
                offset = skip_sc(get_next_token(offset + 1), offset, &get_next_token);
                token = get_next_token(offset);
                return check_integer::<F>(&token, 0, DISALLOW_SIGN, offset);
            } else {
                return check_integer::<F>(&token, i + 2, DISALLOW_SIGN, offset);
            }
        }
    }

    0
}

use crate::tokenizer::char_code_definitions::{
    DIGIT_CATEGORY, NAME_START_CATEGORY, NON_PRINTABLE_CATEGORY, WHITE_SPACE_CATEGORY,
    char_code_category, is_bom, is_identifier_start, is_name, is_newline, is_number_start,
    is_valid_escape,
};
use crate::tokenizer::types::*;
use crate::tokenizer::utils::{
    cmp_str, consume_bad_url_remnants, consume_escaped, consume_name, consume_number,
    find_white_space_end, get_newline_length,
};

pub fn tokenize<F>(source: &str, mut on_token: F)
where
    F: FnMut(u8, usize, usize),
{
    let source = source.to_string();
    let source_length = source.len();
    let mut start = if is_bom(source.chars().next().unwrap_or('\0') as u32) {
        1
    } else {
        0
    };
    let mut offset = start;
    let mut token_type: u8;

    fn get_char_code(source: &str, offset: usize) -> u32 {
        source.chars().nth(offset).map(|c| c as u32).unwrap_or(0)
    }

    fn consume_numeric_token(source: &str, offset: &mut usize) -> u8 {
        *offset = consume_number(source, *offset);

        if is_identifier_start(
            get_char_code(source, *offset),
            get_char_code(source, *offset + 1),
            get_char_code(source, *offset + 2),
        ) {
            *offset = consume_name(source, *offset);
            return DIMENSION;
        }

        if get_char_code(source, *offset) == 0x0025 {
            *offset += 1;
            return PERCENTAGE;
        }

        NUMBER
    }

    fn consume_ident_like_token(source: &str, offset: &mut usize) -> u8 {
        let name_start_offset = *offset;
        *offset = consume_name(source, *offset);

        if cmp_str(source, name_start_offset, *offset, "url")
            && get_char_code(source, *offset) == 0x0028
        {
            *offset = find_white_space_end(source, *offset + 1);

            if matches!(get_char_code(source, *offset), 0x0022 | 0x0027) {
                *offset = name_start_offset + 4;
                return FUNCTION;
            }

            consume_url_token(source, offset);
            return URL;
        }

        if get_char_code(source, *offset) == 0x0028 {
            *offset += 1;
            return FUNCTION;
        }

        IDENT
    }

    fn consume_string_token(
        source: &str,
        offset: &mut usize,
        ending_code_point: Option<u32>,
    ) -> u8 {
        let mut ending_code_point = ending_code_point.unwrap_or_else(|| {
            let code = get_char_code(source, *offset);
            *offset += 1;
            code
        });

        for i in *offset..source.len() {
            let code = get_char_code(source, i);

            match char_code_category(code) {
                c if c == ending_code_point => {
                    *offset = i + 1;
                    return STRING;
                }
                WHITE_SPACE_CATEGORY if is_newline(code) => {
                    *offset += get_newline_length(source, *offset, code);
                    return BAD_STRING;
                }
                0x005C => {
                    if i == source.len() - 1 {
                        break;
                    }

                    let next_code = get_char_code(source, i + 1);
                    if is_newline(next_code) {
                        *offset += get_newline_length(source, i + 1, next_code);
                    } else if is_valid_escape(code, next_code) {
                        *offset = consume_escaped(source, i) - 1;
                    }
                }
                _ => {}
            }
        }

        STRING
    }

    fn consume_url_token(source: &str, offset: &mut usize) -> u8 {
        *offset = find_white_space_end(source, *offset);

        for i in *offset..source.len() {
            let code = get_char_code(source, i);

            match char_code_category(code) {
                0x0029 => {
                    *offset = i + 1;
                    return URL;
                }
                WHITE_SPACE_CATEGORY => {
                    *offset = find_white_space_end(source, i);

                    if get_char_code(source, *offset) == 0x0029 || *offset >= source.len() {
                        if *offset < source.len() {
                            *offset += 1;
                        }
                        return URL;
                    }

                    *offset = consume_bad_url_remnants(source, *offset);
                    return BAD_URL;
                }
                0x0022 | 0x0027 | 0x0028 | NON_PRINTABLE_CATEGORY => {
                    *offset = consume_bad_url_remnants(source, i);
                    return BAD_URL;
                }
                0x005C => {
                    if is_valid_escape(code, get_char_code(source, i + 1)) {
                        *offset = consume_escaped(source, i) - 1;
                    } else {
                        *offset = consume_bad_url_remnants(source, i);
                        return BAD_URL;
                    }
                }
                _ => {}
            }
        }

        URL
    }

    while offset < source_length {
        let code = get_char_code(&source, offset);

        token_type = match char_code_category(code) {
            WHITE_SPACE_CATEGORY => {
                offset = find_white_space_end(&source, offset + 1);
                WHITE_SPACE
            }
            0x0022 => consume_string_token(&source, &mut offset, None),
            0x0023 => {
                if is_name(get_char_code(&source, offset + 1))
                    || is_valid_escape(
                        get_char_code(&source, offset + 1),
                        get_char_code(&source, offset + 2),
                    )
                {
                    offset = consume_name(&source, offset + 1);
                    HASH
                } else {
                    offset += 1;
                    DELIM
                }
            }
            0x0027 => consume_string_token(&source, &mut offset, None),
            0x0028 => {
                offset += 1;
                LEFT_PARENTHESIS
            }
            0x0029 => {
                offset += 1;
                RIGHT_PARENTHESIS
            }
            0x002B => {
                if is_number_start(
                    code,
                    get_char_code(&source, offset + 1),
                    get_char_code(&source, offset + 2),
                ) {
                    consume_numeric_token(&source, &mut offset)
                } else {
                    offset += 1;
                    DELIM
                }
            }
            0x002C => {
                offset += 1;
                COMMA
            }
            0x002D => {
                if is_number_start(
                    code,
                    get_char_code(&source, offset + 1),
                    get_char_code(&source, offset + 2),
                ) {
                    consume_numeric_token(&source, &mut offset)
                } else if get_char_code(&source, offset + 1) == 0x002D
                    && get_char_code(&source, offset + 2) == 0x003E
                {
                    offset += 3;
                    CDC
                } else if is_identifier_start(
                    code,
                    get_char_code(&source, offset + 1),
                    get_char_code(&source, offset + 2),
                ) {
                    consume_ident_like_token(&source, &mut offset)
                } else {
                    offset += 1;
                    DELIM
                }
            }
            0x002E => {
                if is_number_start(
                    code,
                    get_char_code(&source, offset + 1),
                    get_char_code(&source, offset + 2),
                ) {
                    consume_numeric_token(&source, &mut offset)
                } else {
                    offset += 1;
                    DELIM
                }
            }
            _ => {
                offset += 1;
                DELIM
            }
        };

        on_token(token_type, start, offset);
        start = offset;
    }
}

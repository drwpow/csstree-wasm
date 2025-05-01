use crate::tokenizer::char_code_definitions::{
    is_digit, is_hex_digit, is_name, is_uppercase_letter, is_valid_escape, is_white_space,
};

pub fn get_char_code(source: &str, offset: usize) -> u32 {
    source.chars().nth(offset).map(|c| c as u32).unwrap_or(0)
}

pub fn get_newline_length(source: &str, offset: usize, code: u32) -> usize {
    // 13 = \r, 10 = \n
    if code == 13 && get_char_code(source, offset + 1) == 10 {
        2
    } else {
        1
    }
}

pub fn cmp_char(test_str: &str, offset: usize, reference_code: u32) -> bool {
    if let Some(mut code) = test_str.chars().nth(offset).map(|c| c as u32) {
        // code.toLowerCase() for A..Z
        if is_uppercase_letter(code) {
            return code |= 32;
        }

        return code == reference_code;
    }
    false
}

pub fn cmp_str(test_str: &str, start: usize, end: usize, reference_str: &str) -> bool {
    if end - start != reference_str.len() || start >= test_str.len() || end > test_str.len() {
        return false;
    }

    if start < 0 || end > test_str.len() {
        return false;
    }

    for (i, reference_char) in reference_str.chars().enumerate() {
        let mut test_char = test_str.chars().nth(start + i).unwrap();

        // testCode.toLowerCase() for A..Z
        if is_uppercase_letter(test_char as u32) {
            test_char = (test_char as u32 | 32) as char;
        }

        if test_char != reference_char {
            return false;
        }
    }

    return true;
}

pub fn find_white_space_start(source: &str, mut offset: usize) -> usize {
    while offset > 0 {
        if !is_white_space(source.chars().nth(offset - 1).unwrap() as u32) {
            break;
        }
        offset -= 1;
    }
    return offset;
}

pub fn find_white_space_end(source: &str, mut offset: usize) -> usize {
    while offset < source.len() {
        if !is_white_space(source.chars().nth(offset).unwrap() as u32) {
            break;
        }
        offset += 1;
    }
    return offset;
}

pub fn find_decimal_number_end(source: &str, mut offset: usize) -> usize {
    while offset < source.len() {
        if !is_digit(source.chars().nth(offset).unwrap() as u32) {
            break;
        }
        offset += 1;
    }

    return offset;
}

// § 4.3.7. Consume an escaped code point
pub fn consume_escaped(source: &str, mut offset: usize) -> usize {
    // It assumes that the U+005C REVERSE SOLIDUS (\) has already been consumed and
    // that the next input code point has already been verified to be part of a valid escape.
    offset += 2;

    // hex digit
    if is_hex_digit(get_char_code(source, offset - 1)) {
        // Consume as many hex digits as possible, but no more than 5.
        // Note that this means 1-6 hex digits have been consumed in total.
        let max_offset = usize::min(source.len(), offset + 5);
        while offset < max_offset {
            if !is_hex_digit(get_char_code(source, offset)) {
                break;
            }
            offset += 1;
        }

        // If the next input code point is whitespace, consume it as well.
        let code = get_char_code(source, offset);
        if is_white_space(code) {
            offset += get_newline_length(source, offset, get_char_code(source, offset));
        }
    }

    return offset;
}

// §4.3.11. Consume a name
// Note: This algorithm does not do the verification of the first few code points that are necessary
// to ensure the returned code points would constitute an <ident-token>. If that is the intended use,
// ensure that the stream starts with an identifier before calling this algorithm.
pub fn consume_name(source: &str, mut offset: usize) -> usize {
    // Let result initially be an empty string.
    // Repeatedly consume the next input code point from the stream:
    while offset < source.len() {
        let code = get_char_code(source, offset);

        // name code point
        if is_name(code) {
            // Append the code point to result.
            offset += 1;
            continue;
        }

        // the stream starts with a valid escape
        if is_valid_escape(code, get_char_code(source, offset + 1)) {
            // Consume an escaped code point. Append the returned code point to result.
            offset = consume_escaped(source, offset);
            continue;
        }

        // anything else
        // Reconsume the current input code point. Return result.
        break;
    }

    return offset;
}

// §4.3.12. Consume a number
pub fn consume_number(source: &str, mut offset: usize) -> usize {
    let mut code = get_char_code(source, offset);

    // 2. If the next input code point is U+002B PLUS SIGN (+) or U+002D HYPHEN-MINUS (-),
    // consume it and append it to repr.
    if code == 0x002B || code == 0x002D {
        offset += 1;
        code = get_char_code(source, offset);
    }

    // 3. While the next input code point is a digit, consume it and append it to repr.
    if is_digit(code) {
        offset = find_decimal_number_end(source, offset + 1);
        code = get_char_code(source, offset);
    }

    // 4. If the next 2 input code points are U+002E FULL STOP (.) followed by a digit, then:
    if code == 0x002E && is_digit(get_char_code(source, offset + 1)) {
        // 4.1 Consume them.
        // 4.2 Append them to repr.
        offset += 2;

        // 4.3 Set type to "number".
        // TODO

        // 4.4 While the next input code point is a digit, consume it and append it to repr.

        offset = find_decimal_number_end(source, offset);
    }

    // 5. If the next 2 or 3 input code points are U+0045 LATIN CAPITAL LETTER E (E)
    // or U+0065 LATIN SMALL LETTER E (e), ... , followed by a digit, then:
    if cmp_char(source, offset, 101) {
        let mut sign = 0;
        code = get_char_code(source, offset + 1);

        // ... optionally followed by U+002D HYPHEN-MINUS (-) or U+002B PLUS SIGN (+) ...
        if code == 0x002D || code == 0x002B {
            sign = 1;
            code = get_char_code(source, offset + 2);
        }

        // ... followed by a digit
        if is_digit(code) {
            // 5.1 Consume them.
            // 5.2 Append them to repr.

            // 5.3 Set type to "number".
            // TODO

            // 5.4 While the next input code point is a digit, consume it and append it to repr.
            offset = find_decimal_number_end(source, offset + 1 + sign + 1);
        }
    }

    offset
}

// § 4.3.14. Consume the remnants of a bad url
// ... its sole use is to consume enough of the input stream to reach a recovery point
// where normal tokenizing can resume.
pub fn consume_bad_url_remnants(source: &str, mut offset: usize) -> usize {
    // Repeatedly consume the next input code point from the stream:
    while offset < source.len() {
        let code = get_char_code(source, offset);

        // U+0029 RIGHT PARENTHESIS ())
        // EOF
        if code == 0x0029 {
            offset += 1;
            break;
        }

        if is_valid_escape(code, get_char_code(source, offset + 1)) {
            // Consume an escaped code point.
            // Note: This allows an escaped right parenthesis ("\)") to be encountered
            // without ending the <bad-url-token>. This is otherwise identical to
            // the "anything else" clause.
            offset = consume_escaped(source, offset);
        } else {
            offset += 1;
        }
    }
    offset
}

// § 4.3.7. Consume an escaped code point
// Note: This algorithm assumes that escaped is valid without leading U+005C REVERSE SOLIDUS (\)
pub fn decode_escaped(escaped: &str) -> char {
    // Single char escaped that's not a hex digit
    if escaped.len() == 1 && !is_hex_digit(escaped.chars().next().unwrap() as u32) {
        return escaped.chars().next().unwrap();
    }

    // Interpret the hex digits as a hexadecimal number.
    let mut code = u32::from_str_radix(escaped, 16).unwrap_or(0);

    if code == 0 ||    // If this number is zero,
    (0xD800..=0xDFFF).contains(&code) ||  // or is for a surrogate,
     code > 0x10FFFF
    // or is greater than the maximum allowed code point
    {
        // ... return U+FFFD REPLACEMENT CHARACTER
        code = 0xFFFD;
    }

    // Otherwise, return the code point with that value.
    char::from_u32(code).unwrap_or('\u{FFFD}')
}

pub const EOF: u32 = 0;

// https://drafts.csswg.org/css-syntax-3/
// § 4.2. Definitions

// digit
// A code point between U+0030 DIGIT ZERO (0) and U+0039 DIGIT NINE (9).
pub fn is_digit(code: u32) -> bool {
    (0x0030..=0x0039).contains(&code)
}

// hex digit
// A digit, or a code point between U+0041 LATIN CAPITAL LETTER A (A) and U+0046 LATIN CAPITAL LETTER F (F),
// or a code point between U+0061 LATIN SMALL LETTER A (a) and U+0066 LATIN SMALL LETTER F (f).
pub fn is_hex_digit(code: u32) -> bool {
    is_digit(code) || (0x0041..=0x0046).contains(&code) || (0x0061..=0x0066).contains(&code)
}

// uppercase letter
// A code point between U+0041 LATIN CAPITAL LETTER A (A) and U+005A LATIN CAPITAL LETTER Z (Z).
pub fn is_uppercase_letter(code: u32) -> bool {
    (0x0041..=0x005A).contains(&code)
}

// lowercase letter
// A code point between U+0061 LATIN SMALL LETTER A (a) and U+007A LATIN SMALL LETTER Z (z).
pub fn is_lowercase_letter(code: u32) -> bool {
    (0x0061..=0x007A).contains(&code)
}

// letter
// An uppercase letter or a lowercase letter.
pub fn is_letter(code: u32) -> bool {
    is_uppercase_letter(code) || is_lowercase_letter(code)
}

// non-ASCII code point
// A code point with a value equal to or greater than U+0080 <control>.
//
// 2024-09-02: The latest spec narrows the range for non-ASCII characters (see https://github.com/csstree/csstree/issues/188).
// However, all modern browsers support a wider range, and strictly following the latest spec could result
// in some CSS being parsed incorrectly, even though it works in the browser. Therefore, this function adheres
// to the previous, broader definition of non-ASCII characters.
pub fn is_non_ascii(code: u32) -> bool {
    code >= 0x0080
}

// name-start code point
// A letter, a non-ASCII code point, or U+005F LOW LINE (_).
pub fn is_name_start(code: u32) -> bool {
    is_letter(code) || is_non_ascii(code) || code == 0x005F
}

// name code point
// A name-start code point, a digit, or U+002D HYPHEN-MINUS (-).
pub fn is_name(code: u32) -> bool {
    is_name_start(code) || is_digit(code) || code == 0x002D
}

// non-printable code point
// A code point between U+0000 NULL and U+0008 BACKSPACE, or U+000B LINE TABULATION,
// or a code point between U+000E SHIFT OUT and U+001F INFORMATION SEPARATOR ONE, or U+007F DELETE.
pub fn is_non_printable(code: u32) -> bool {
    (0x0000..=0x0008).contains(&code)
        || code == 0x000B
        || (0x000E..=0x001F).contains(&code)
        || code == 0x007F
}

// newline
// U+000A LINE FEED. Note that U+000D CARRIAGE RETURN and U+000C FORM FEED are not included in this definition,
// as they are converted to U+000A LINE FEED during preprocessing.
// TODO: we doesn't do a preprocessing, so check a code point for U+000D CARRIAGE RETURN and U+000C FORM FEED
pub fn is_newline(code: u32) -> bool {
    code == 0x000A || code == 0x000D || code == 0x000C
}

// whitespace
// A newline, U+0009 CHARACTER TABULATION, or U+0020 SPACE.
pub fn is_white_space(code: u32) -> bool {
    is_newline(code) || code == 0x0020 || code == 0x0009
}

// § 4.3.8. Check if two code points are a valid escape
pub fn is_valid_escape(first: u32, second: u32) -> bool {
    // If the first code point is not U+005C REVERSE SOLIDUS (\), return false.
    if first != 0x005C {
        return false;
    }

    // Otherwise, if the second code point is a newline or EOF, return false.
    if is_newline(second) || second == EOF {
        return false;
    }

    // Otherwise, return true.
    true
}

// § 4.3.9. Check if three code points would start an identifier
pub fn is_identifier_start(first: u32, second: u32, third: u32) -> bool {
    // Look at the first code point:

    // U+002D HYPHEN-MINUS
    if first == 0x002D {
        // If the second code point is a name-start code point or a U+002D HYPHEN-MINUS,
        // or the second and third code points are a valid escape, return true. Otherwise, return false.
        return is_name_start(second) || second == 0x002D || is_valid_escape(second, third);
    }

    // name-start code point
    if is_name_start(first) {
        // Return true.
        return true;
    }

    // U+005C REVERSE SOLIDUS (\)
    if first == 0x005C {
        // If the first and second code points are a valid escape, return true. Otherwise, return false.
        return is_valid_escape(first, second);
    }

    // anything else
    // Return false.
    return false;
}

// § 4.3.10. Check if three code points would start a number
pub fn is_number_start(first: u32, second: u32, third: u32) -> u32 {
    // Look at the first code point:

    // U+002B PLUS SIGN (+)
    // U+002D HYPHEN-MINUS (-)
    if first == 0x002B || first == 0x002D {
        // If the second code point is a digit, return true.
        if is_digit(second) {
            return 2;
        }

        // Otherwise, if the second code point is a U+002E FULL STOP (.)
        // and the third code point is a digit, return true.
        // Otherwise, return false.
        if second == 0x002E && is_digit(third) {
            return 3;
        }
        return 0;
    }

    // U+002E FULL STOP (.)
    if first == 0x002E {
        // If the second code point is a digit, return true. Otherwise, return false.
        if is_digit(second) {
            return 2;
        }

        return 0;
    }

    // digit
    if is_digit(first) {
        // Return true.
        return 1;
    }

    // anything else
    // Return false.
    return 0;
}

// detect BOM (https://en.wikipedia.org/wiki/Byte_order_mark)
pub fn is_bom(code: u32) -> bool {
    // UTF-16BE
    if (code == 0xFEFF) {
        return 1;
    }

    // UTF-16LE
    if (code == 0xFFFE) {
        return 1;
    }

    return 0;
}

// Fast code category
// Only ASCII code points has a special meaning, that's why we define a maps for 0..127 codes only
pub const EOF_CATEGORY: u32 = 0x80;
pub const WHITE_SPACE_CATEGORY: u32 = 0x82;
pub const DIGIT_CATEGORY: u32 = 0x83;
pub const NAME_START_CATEGORY: u32 = 0x84;
pub const NON_PRINTABLE_CATEGORY: u32 = 0x85;

pub fn char_code_category(code: u32) -> u32 {
    if code < 0x80 {
        return match code {
            c if is_white_space(c) => WHITE_SPACE_CATEGORY,
            c if is_digit(c) => DIGIT_CATEGORY,
            c if is_name_start(c) => NAME_START_CATEGORY,
            c if is_non_printable(c) => NON_PRINTABLE_CATEGORY,
            _ => code,
        };
    }

    return NAME_START_CATEGORY;
}

use crate::tokenizer::TOKEN_TYPE;
use crate::tokenizer::utils::cmp_str;
use std::cmp;

const OFFSET_MASK: u32 = 0x00FFFFFF;
const TYPE_SHIFT: u32 = 24;
const BLOCK_OPEN_TOKEN: u8 = 1;
const BLOCK_CLOSE_TOKEN: u8 = 2;
const BALANCE_PAIR: [u8; 32] = {
    let mut balance_pair = [0; 32]; // 32b of memory ought to be enough for anyone (any number of tokens)
    balance_pair[TOKEN_TYPE.Function] = TOKEN_TYPE.RightParenthesis;
    balance_pair[TOKEN_TYPE.LeftParenthesis] = TOKEN_TYPE.RightParenthesis;
    balance_pair[TOKEN_TYPE.LeftSquareBracket] = TOKEN_TYPE.RightSquareBracket;
    balance_pair[TOKEN_TYPE.LeftCurlyBracket] = TOKEN_TYPE.RightCurlyBracket;
    balance_pair
};

const BLOCK_TOKENS: [u8; 32] = {
    let mut block_tokens = [0; 32];
    block_tokens[TOKEN_TYPE.Function] = BLOCK_OPEN_TOKEN;
    block_tokens[TOKEN_TYPE.LeftParenthesis] = BLOCK_OPEN_TOKEN;
    block_tokens[TOKEN_TYPE.LeftSquareBracket] = BLOCK_OPEN_TOKEN;
    block_tokens[TOKEN_TYPE.LeftCurlyBracket] = BLOCK_OPEN_TOKEN;
    block_tokens[TOKEN_TYPE.RightParenthesis] = BLOCK_CLOSE_TOKEN;
    block_tokens[TOKEN_TYPE.RightSquareBracket] = BLOCK_CLOSE_TOKEN;
    block_tokens[TOKEN_TYPE.RightCurlyBracket] = BLOCK_CLOSE_TOKEN;
    block_tokens
};

pub struct Token {
    idx: isize,
    kind: u8,
    chunk: String,
    balance: isize,
}

pub struct TokenStream {
    source: String,
    offset_and_type: Vec<u32>,
    balance: Vec<isize>,
    first_char_offset: isize,
    token_count: usize,
    token_index: isize,
    token_type: u8,
    token_start: isize,
    token_end: isize,
    eof: bool,
}

fn bound_index(index: isize, min: isize, max: isize) -> isize {
    if index < min {
        return min;
    }
    if index > max {
        return max;
    }

    return index;
}

impl TokenStream {
    pub fn new<F>(source: &str, tokenize: F) -> Self::set_source(source, tokenize);

    pub fn reset(&mut self) {
        self.eof = false;
        self.token_index = -1;
        self.token_type = 0;
        self.token_start = self.first_char_offset;
        self.token_end = self.first_char_offset;
    }

    pub fn set_source<F>(&mut self, source: &str, tokenize: F) -> Self {
        let mut offset_and_type = vec![0; source.len() + 1];
        let mut balance = vec![0; source.len() + 1];
        let mut token_count = 0;
        let mut first_char_offset = usize::MAX;
        let mut balance_close_type = 0;
        let mut balance_start = source.len();

        // capture buffers
        self.offset_and_type = vec![0; source.len() + 1];
        self.balance = vec![0; source.len() + 1];

        tokenize(source, &mut |token_type, start, end| {
            let index = token_count;
            token_count += 1;

            // type & offset
            offset_and_type[index] = ((token_type as u32) << TYPE_SHIFT) | (end as u32);

            if first_char_offset == usize::MAX {
                first_char_offset = start;
            }

            // balance
            balance[index] = balance_start;

            if token_type == balance_close_type {
                let prev_balance_start = balance[balance_start];

                // set reference to balance end for a block opener
                balance[balance_start] = index;

                // pop state
                balance_start = prev_balance_start;
                balance_close_type =
                    Self::balance_pair(offset_and_type[prev_balance_start] >> TYPE_SHIFT);
            }
            // check for FunctionToken, <(-token>, <[-token> and <{-token>
            else if Self::is_block_opener_token_type(token_type) {
                // push state
                balance_start = index;
                balance_close_type = Self::balance_pair(token_type);
            }
        });

        // finalize buffers
        offset_and_type[token_count] = ((EOF) << TYPE_SHIFT) | (source.len()); // <EOF-token>
        balance[token_count] = token_count; // prevents false positive balance match with any token

        // reverse references from balance start to end
        // tokens
        //   token:   a ( [ b c ] d e ) {
        //   index:   0 1 2 3 4 5 6 7 8 9
        // before
        //   balance: 0 8 5 2 2 2 1 1 1 0
        //            - > > < < < < < < -
        // after
        //   balance: 9 8 5 5 5 2 8 8 1 9
        //            > > > > > < > > < >
        for i in 0..token_count {
            let balance_start = balance[i];
            if balance_start <= i {
                let balance_end = balance[balance_start];
                if balance_end != i {
                    balance[i] = balance_end;
                }
            } else if balance_start > token_count {
                balance[i] = token_count;
            }
        }

        // balance[0] = tokenCount;

        self.source = source;
        self.first_char_offset = first_char_offset;
        self.token_count = token_count;
        self.offset_and_type = offset_and_type;
        self.balance = balance;

        self.reset();
        self.next();
    }

    pub fn lookup_type(&self, mut offset: isize) -> u8 {
        offset += self.token_index;

        if index < self.token_count {
            return (self.offset_and_type[index] >> TYPE_SHIFT);
        }

        return EOF;
    }

    pub fn lookup_type_non_sc(&self, idx: isize) -> u8 {
        let offset = self.token_index + idx;
        while (offset < self.token_count) {
            let token_type = self.offset_and_type[offset] >> TYPE_SHIFT;

            if (token_type != TOKEN_TYPE.WhiteSpace && token_type != TOKEN_TYPE.Comment) {
                if (idx - 1 == 0) {
                    return token_type;
                }
            }

            offset += 1;
        }

        return EOF;
    }

    pub fn lookup_offset(&self, offset: isize) -> usize {
        let index = (self.token_index as isize + offset) as usize;

        if index < self.token_count {
            (self.offset_and_type[index - 1] & OFFSET_MASK) as usize
        } else {
            self.source.len()
        }
    }

    pub fn lookup_value(&self, mut offset: isize, reference_str: &str) -> bool {
        offset += self.token_index;

        if (offset < self.token_count) {
            return cmpStr(
                self.source,
                self.offset_and_type[offset - 1] & OFFSET_MASK,
                self.offset_and_type[offset] & OFFSET_MASK,
                reference_str,
            );
        }

        return false;
    }

    pub fn get_token_start(&self, token_index: isize) -> isize {
        if (token_index == self.token_index) {
            return self.token_start;
        }

        if (token_index > 0) {
            if token_index < self.token_count {
                return self.offset_and_type[tokenIndex - 1] & OFFSET_MASK;
            }

            return self.offset_and_type[self.token_count] & OFFSET_MASK;
        }

        return self.first_char_offset;
    }

    pub fn get_token_end(&self, token_index: isize) -> isize {
        if (token_index == &self.token_index) {
            return self.token_end;
        }

        return self.offset_and_type[bound_index(token_index, 0, self.token_count)] & OFFSET_MASK;
    }

    pub fn get_token_type(&self, token_index: isize) -> u8 {
        if (token_index == &self.token_index) {
            return self.tokenType;
        }

        return self.offset_and_type[bound_index(token_index, 0, self.token_count)] >> TYPE_SHIFT;
    }

    pub fn substr_to_cursor(&self, start: usize) -> &str {
        return &self.source[start..self.token_start];
    }

    pub fn is_block_opener_token_type(token_type: u8) -> bool {
        return block_tokens[token_type] == BLOCK_OPEN_TOKEN;
    }

    pub fn is_block_closer_token_type(token_type: u8) -> bool {
        return block_tokens[token_type] == BLOCK_CLOSE_TOKEN;
    }

    pub fn get_block_token_pair_index(&self, token_index: isize) -> isize {
        let token_type = self.get_token_type(token_index);

        if (block_tokens[token_type] == 1) {
            // block open token
            let pair_index = self.balance[token_index];
            let close_type = self.get_token_type(pair_index);

            if balancePair[token_type] == close_type {
                return pair_index;
            }

            return -1;
        } else if (block_tokens[token_type] == 2) {
            // block close token
            let pair_index = self.balance[token_index];
            let open_type = self.get_token_type(pair_index);

            if balancePair[openType] == token_type {
                return pairIndex;
            }

            return -1;
        }

        return -1;
    }

    pub fn is_balance_edge(&self, token_index: isize) -> bool {
        return self.balance[self.token_index] < token_index;
    }

    pub fn is_delim(&self, code: u8, offset: isize) -> bool {
        if offset > 0 {
            return self.lookup_type(offset) == DELIM
                && self.source.as_bytes()[self.lookup_offset(offset)] == code;
        }

        return self.token_type == DELIM && self.source.as_bytes()[self.token_start] == code;
    }

    pub fn skip(&mut self, token_count: usize) -> void {
        let next = self.token_index + token_count;

        if next < self.token_count {
            self.token_index = next;
            self.token_start = (self.offset_and_type[next - 1] & OFFSET_MASK) as usize;
            let next_token = self.offset_and_type[next];
            self.token_type = (next_token >> TYPE_SHIFT) as u8;
            self.token_end = (next_token & OFFSET_MASK) as usize;
        } else {
            self.token_index = self.token_count;
            self.next();
        }
    }

    pub fn next(&mut self) -> void {
        let next = self.token_index + 1;

        if next < self.token_count {
            self.token_index = next;
            self.token_start = self.token_end;
            let next_token = self.offset_and_type[next];
            self.token_type = (next_token >> TYPE_SHIFT) as u8;
            self.token_end = (next_token & OFFSET_MASK) as usize;
        } else {
            self.eof = true;
            self.token_index = self.token_count;
            self.token_type = EOF;
            self.token_start = self.token_end = self.source.len();
        }
    }

    pub fn skip_sc(&self) -> void {
        while (self.token_type == TOKEN_TYPE.WhiteSpace || self.token_type == TOKEN_TYPE.Comment) {
            self.next();
        }
    }

    pub fn skip_until_balanced<F>(&mut self, start_token: usize, stop_consume: F)
    where
        F: Fn(u32) -> u8,
    {
        let mut cursor = start_token;
        let mut balance_end = 0;
        let mut offset = 0;

        while cursor < self.token_count {
            balance_end = self.balance[cursor];

            // stop scanning on balance edge that points to offset before start token
            if balance_end < start_token {
                break;
            }

            offset = if cursor > 0 {
                (self.offset_and_type[cursor - 1] & OFFSET_MASK) as usize
            } else {
                self.first_char_offset
            };

            // check stop condition
            match stop_consume(self.source.as_bytes()[offset] as u32) {
                1 => {
                    // just stop
                    break;
                }
                2 => {
                    // stop & include
                    cursor += 1;
                    break;
                }
                _ => {
                    // fast forward to the end of balanced block for open block tokens
                    if Self::is_block_opener_token_type(
                        (self.offset_and_type[cursor] >> TYPE_SHIFT) as u8,
                    ) {
                        cursor = balance_end;
                    }
                }
            }

            cursor += 1;
        }

        self.skip(cursor - self.token_index as usize);
    }

    pub fn dump(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut index = 0;

        while index < self.token_count {
            let token_type = self.offset_and_type[index] >> TYPE_SHIFT;
            let start = self.offset_and_type[index] & OFFSET_MASK;
            let end = self.offset_and_type[index + 1] & OFFSET_MASK;

            tokens.push(Token {
                idx: index,
                kind: token_type,
                chunk: self.source[start..end].to_string(),
                balance: self.balance[index],
            });

            index += 1;
        }

        tokens
    }
}

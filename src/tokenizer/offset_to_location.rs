use crate::tokenizer::char_code_definitions::is_bom;

const N: u8 = 10; // \n
const F: u8 = 12; // \f
const R: u8 = 13; // \r

pub struct OffsetToLocation {
    source: String,
    start_offset: usize,
    start_line: usize,
    start_column: usize,
    lines: Vec<usize>,
    columns: Vec<usize>,
    computed: bool,
}

impl OffsetToLocation {
    pub fn new(
        source: String,
        start_offset: usize,
        start_line: usize,
        start_column: usize,
    ) -> Self {
        let mut instance = OffsetToLocation {
            source,
            start_offset,
            start_line,
            start_column,
            lines: Vec::new(),
            columns: Vec::new(),
            computed: false,
        };
        instance.set_source(
            instance.source.clone(),
            start_offset,
            start_line,
            start_column,
        );
        instance
    }

    pub fn set_source(
        &mut self,
        source: String,
        start_offset: usize,
        start_line: usize,
        start_column: usize,
    ) {
        self.source = source;
        self.start_offset = start_offset;
        self.start_line = start_line;
        self.start_column = start_column;
        self.computed = false;
    }

    pub fn get_location(&mut self, offset: usize, filename: &str) -> Location {
        if !self.computed {
            self.compute_lines_and_columns();
        }

        Location {
            source: filename.to_string(),
            offset: self.start_offset + offset,
            line: self.lines[offset],
            column: self.columns[offset],
        }
    }

    pub fn get_location_range(
        &mut self,
        start: usize,
        end: usize,
        filename: &str,
    ) -> LocationRange {
        if !self.computed {
            self.compute_lines_and_columns();
        }

        LocationRange {
            source: filename.to_string(),
            start: Location {
                source: None,
                offset: self.start_offset + start,
                line: self.lines[start],
                column: self.columns[start],
            },
            end: Location {
                source: None,
                offset: self.start_offset + end,
                line: self.lines[end],
                column: self.columns[end],
            },
        }
    }

    fn compute_lines_and_columns(&mut self) {
        let source = self.source.as_bytes();
        let source_length = source.len();
        let mut start_offset: usize = 0;
        if source_length > 0 && is_bom(source[0] as u32) {
            start_offset = 1
        }
        self.lines = vec![0; source_length + 1];
        self.columns = vec![0; source_length + 1];
        let mut line = self.start_line;
        let mut column = self.start_column;

        for i in start_offset..source_length {
            let code: u8 = source[i];

            self.lines[i] = line;
            self.columns[i] = column;

            column += 1;

            if code == N || code == R || code == F {
                if code == R && i + 1 < source_length && source[i + 1] as u32 == N {
                    self.lines[i + 1] = line;
                    self.columns[i + 1] = column;
                }

                line += 1;
                column = 1;
            }
        }

        self.lines[source_length] = line;
        self.columns[source_length] = column;

        self.computed = true;
    }
}

#[derive(Debug)]
pub struct Location {
    pub source: Option<String>,
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug)]
pub struct LocationRange {
    pub source: String,
    pub start: Location,
    pub end: Location,
}

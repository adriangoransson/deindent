use std::io::{Result, Write};

#[derive(Clone, Copy)]
/// Indent information from a scanned string.
pub struct IndentInfo {
    /// Index of first non-empty line.
    pub first_line: usize,
    /// Index of last non-empty line.
    pub last_line: usize,
    /// Least number of whitespace character that can be stripped from each line to deindent.
    pub leading_whitespace: usize,
}

impl IndentInfo {
    /// Returns `None` on empty or whitespace-only input.
    pub fn new(input: &str) -> Option<IndentInfo> {
        let mut first_char_line = None;
        let mut last_char_line = 0;
        let mut common: Option<usize> = None;

        for (index, line) in input.lines().enumerate() {
            let ws = line
                .bytes()
                .take_while(|&c| c.is_ascii_whitespace())
                .count();

            if ws == line.len() {
                continue;
            }

            first_char_line.get_or_insert(index);
            last_char_line = index;

            common = match common {
                None => Some(ws),
                Some(x) => Some(x.min(ws)),
            };
        }

        first_char_line.map(|x| IndentInfo {
            first_line: x,
            last_line: last_char_line,
            leading_whitespace: common.unwrap_or(0),
        })
    }
}

/// An [`IndentInfo`] with a reference to the scanned string, making it possible to write a
/// deindented copy.
pub struct Deindenter<'a> {
    indent_info: IndentInfo,
    input: &'a str,
}

impl Deindenter<'_> {
    /// Returns `None` on empty or whitespace-only input.
    pub fn new(input: &str) -> Option<Deindenter> {
        let indent_info = IndentInfo::new(input)?;

        Some(Deindenter { indent_info, input })
    }

    /// Writes the scanned input with leading whitespace removed.
    /// Will return an error if `T::write_all` fails.
    pub fn to_writer<T: Write>(&self, mut out: T) -> Result<()> {
        let IndentInfo {
            first_line,
            last_line,
            leading_whitespace,
        } = self.indent_info;

        for line in self
            .input
            .split_inclusive('\n')
            .skip(first_line)
            .take(1 + last_line - first_line)
        {
            let trim = line.bytes().take(leading_whitespace).len();
            let skip = if trim < leading_whitespace {
                0 // Whitespace-only line.
            } else {
                trim
            };

            out.write_all(&line.as_bytes()[skip..])?;
        }

        Ok(())
    }

    /// Writes the scanned input with leading whitespace removed. Allocates a buffer and uses
    /// `to_writer`. Will error on the same conditions.
    ///
    /// # Panics
    ///
    /// Will panic if the buffer cannot be converted to a UTF-8 string. Since the input must be a
    /// valid `&str`, this should never happen.
    pub fn to_string(&self) -> Result<String> {
        let mut buf = Vec::new();
        self.to_writer(&mut buf)?;

        let s = String::from_utf8(buf).expect("deindented string contains non-utf8 text");
        Ok(s)
    }

    /// Copy of the underlying [`IndentInfo`].
    pub fn indent_info(&self) -> IndentInfo {
        self.indent_info
    }
}

impl From<Deindenter<'_>> for IndentInfo {
    fn from(value: Deindenter) -> Self {
        value.indent_info
    }
}

#[cfg(test)]
mod tests {
    use crate::Deindenter;

    fn test(input: &str, expected: &str) {
        let d = Deindenter::new(input).unwrap();
        assert_eq!(d.to_string().unwrap(), expected);
    }

    const EXPECTED: &str = r#"impl From<Deindenter<'_>> for IndentInfo {
    fn from(value: Deindenter) -> Self {
        value.indent_info
    }
}
"#;

    #[test]
    fn extra_whitespace_lines() {
        let input = r#"

impl From<Deindenter<'_>> for IndentInfo {
    fn from(value: Deindenter) -> Self {
        value.indent_info
    }
}

"#;

        test(input, EXPECTED);
    }

    #[test]
    fn noop() {
        let input = r#"impl From<Deindenter<'_>> for IndentInfo {
    fn from(value: Deindenter) -> Self {
        value.indent_info
    }
}
"#;

        test(input, input);
    }

    #[test]
    fn indented() {
        let input = r#"                impl From<Deindenter<'_>> for IndentInfo {
                    fn from(value: Deindenter) -> Self {
                        value.indent_info
                    }
                }
"#;

        test(input, EXPECTED);
    }

    #[test]
    fn almost_indented() {
        // First row is not indented at all == noop.
        let input = r#"impl From<Deindenter<'_>> for IndentInfo {
                    fn from(value: Deindenter) -> Self {
                        value.indent_info
                    }
                }
"#;

        test(input, input);
    }

    #[test]
    fn no_trailing_newline() {
        let input = r#"                impl From<Deindenter<'_>> for IndentInfo {
                    fn from(value: Deindenter) -> Self {
                        value.indent_info
                    }
                }"#;

        let expected = r#"impl From<Deindenter<'_>> for IndentInfo {
    fn from(value: Deindenter) -> Self {
        value.indent_info
    }
}"#;

        test(input, expected);
    }

    #[test]
    fn multiple_paragraphs() {
        let mut input = r#"  
        this is p1

        this is p2"#
            .to_owned();

        let mut expected = r#"this is p1

this is p2"#
            .to_owned();

        test(&input, &expected);

        // With trailing newline.
        input.push('\n');
        expected.push('\n');

        test(&input, &expected);
    }
}

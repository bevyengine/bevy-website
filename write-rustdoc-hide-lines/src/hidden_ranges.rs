use std::ops::Range;

use regex::Regex;

pub type HiddenRanges = Vec<Range<usize>>;

// The generic is to allow both `&[String]` (slice of `Vec<String>`) and `&[&str]` (slice of `Vec<&str>`)
// See: https://stackoverflow.com/a/41180422/379923
pub fn get_hidden_ranges<T: AsRef<str>>(code: &[T]) -> HiddenRanges {
    let mut ranges = vec![];
    let mut curr_range: Option<Range<usize>> = None;

    // Match lines starting with a potentially indented `#` followed by a space or EOL.
    let Ok(is_hidden_re) = Regex::new(r"^\s*#(?: |$)") else {
        return ranges;
    };

    for (idx, line) in code.iter().enumerate() {
        let n = idx + 1;
        let line = line.as_ref();
        let is_hidden = is_hidden_re.is_match(line);

        if is_hidden {
            if let Some(range) = curr_range.as_mut() {
                range.end = n;
            } else {
                curr_range = Some(Range { start: n, end: n });
            }
        } else {
            if let Some(curr_range) = curr_range {
                ranges.push(curr_range);
            }

            curr_range = None;
        }
    }

    if let Some(curr_range) = curr_range {
        ranges.push(curr_range);
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    fn split_lines(code: &str) -> Vec<&str> {
        code.split('\n').collect::<Vec<_>>()
    }

    #[test]
    fn empty_block() {
        let code = split_lines(indoc! {r#""#});

        assert!(get_hidden_ranges(&code).is_empty());
    }

    #[test]
    fn no_hidden() {
        let code = split_lines(indoc! {r#"
            1
            2
            3
            4
            5
        "#});

        assert!(get_hidden_ranges(&code).is_empty());
    }

    #[test]
    fn single_range() {
        let code = split_lines(indoc! {r#"
            # 1
            # 2
            # 3
            4
            5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![Range { start: 1, end: 3 }]);
    }

    #[test]
    fn single_range_no_content() {
        let code = split_lines(indoc! {r#"
            #
            #
            #
            4
            5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![Range { start: 1, end: 3 }]);
    }

    #[test]
    fn multi_range() {
        let code = split_lines(indoc! {r#"
            # 1
            # 2
            3
            # 4
            # 5
        "#});

        assert_eq!(
            get_hidden_ranges(&code),
            vec![Range { start: 1, end: 2 }, Range { start: 4, end: 5 }]
        );
    }

    #[test]
    fn single_line_range() {
        let code = split_lines(indoc! {r#"
            1
            2
            3
            4
            # 5
        "#});

        assert_eq!(get_hidden_ranges(&code), vec![Range { start: 5, end: 5 }]);
    }
}

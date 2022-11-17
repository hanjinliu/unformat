use super::vars;

const FMT_START: str = '{';
const FMT_END: str = '}';
const FMT_SEP: str = ':';

pub struct FormatPattern {
    pub pattern: String,
}

impl FormatPattern {
    fn new(pattern: String) -> FormatPattern {
        FormatPattern { pattern }
    }

    fn name_at(&self, idx: usize) -> String {
        assert_eq!(self.pattern[idx], FMT_START);
        let mut name = String::new();
        let mut iter = self.pattern.chars();
        iter.skip(idx);
        for ch in iter {
            if ch == FMT_END {
                break;
            }
            name.push(ch);
        }
        name
    }

    fn iter_names(&self) {

    }

    fn get_vars(&self, s: &String) {
        let mut active = false;
        let mut current = String::new();
        let mut vars = VarVector::new();
        let mut idx: usize = 0;  // current index on the pattern string
        for c in s.chars() {
            if c == FMT_START {
                active = true;
                idx += 1;
            } else if c == FMT_END {
                active = false;
                idx += 1;
                vars.add()
            } else if active {
                current.push(c);
            }
        }
        
    }
}

struct FormatPatternIter<'a> {
    pattern: &'a FormatPattern,
    idx: usize,
}

impl FormatPatternIter<'_> {
    fn new(pattern: &FormatPattern) -> FormatPatternIter {
        FormatPatternIter { pattern, idx: 0 }
    }
}

impl Iterator for FormatPatternIter<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        if self.idx >= self.pattern.pattern.len() {
            return None;
        }
        let mut name = String::new();
        loop {
            self.idx += 1;
            if self.idx >= self.pattern.pattern.len() {
                return None;
            }
            match self.pattern.pattern.chars().nth(self.idx) {
                Some(FMT_START) => {
                    self.idx += 1;
                    continue;
                }
                Some(FMT_END) => {
                    self.idx += 1;
                    return Some(name);
                }
                Some(c) => {
                    name.push(c);
                }
                None => {
                    return None;
                }
            }

        }
    }
}
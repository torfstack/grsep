pub fn grep(line: &str, p: &Pattern) -> (Vec<usize>, bool) {
    let m = p.pattern.chars().count();
    let n = line.chars().count();
    if n < m {
        return (vec![], false);
    }
    let mut res = vec![];

    let mut k = m-1;
    let mut pk = m-1;
    while k < n {
        if p.pattern.chars().count() > 1 && pk == 0 || p.pattern.chars().count() == 1 && p.pattern.chars().nth(0) == line.chars().nth(k) {
            res.push(k);
            k += m;
            pk = m-1;
        } else if !p.contains(line.chars().nth(k).unwrap()) {
            k += m;
            pk = m-1;
        } else if line.chars().nth(k).unwrap() == p.pattern.chars().nth(pk).unwrap() {
            k -= 1;
            pk -= 1;
        } else {
            k += 1;
        }
    }

    let found = !res.is_empty();
    (res, found)
}


#[derive(Debug)]
pub struct Pattern {
    pattern: String,
    exist: Vec<usize>,
    lookup: Vec<isize>,
}

impl Pattern {
    pub fn new(pattern: &str) -> Pattern {
        let mut p = Pattern {
            pattern: pattern.to_string(),
            exist: vec![0; 256],
            lookup: vec![-1; 256],
        };
        p.preprocess();
        p
    }

    pub fn contains(&self, c: char) -> bool {
        self.exist[c as usize] == 1
    }

    pub fn length(&self) -> usize {
        self.pattern.len()
    }

    fn preprocess(&mut self) {
        let m = self.pattern.len();
        self.pattern.chars()
            .enumerate()
            .into_iter()
            .for_each(|(i, c)| {
                self.exist[c as usize] = 1;
                self.lookup[c as usize] = m as isize - i as isize - 1;
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep() {
        let p = Pattern::new("ab");
        let (indices, found) = grep("ababab", &p);
        assert_eq!(indices, vec![0, 2, 4]);
        assert_eq!(found, true);
    }

    #[test]
    fn test_grep_single_letter_pattern() {
        let p = Pattern::new("b");
        let (indices, found) = grep("ababab", &p);
        assert_eq!(indices, vec![1, 3, 5]);
        assert_eq!(found, true);
    }

    #[test]
    fn test_grep_longer_pattern() {
        let p = Pattern::new("abc");
        let (indices, found) = grep("abdabcabc", &p);
        assert_eq!(indices, vec![3, 6]);
        assert_eq!(found, true);
    }

    #[test]
    fn test_grep_fail() {
        let p = Pattern::new("Zauberhaft");
        let (indices, found) = grep("yyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy", &p);
        assert_eq!(indices, vec![]);
        assert_eq!(found, false);
    }
}
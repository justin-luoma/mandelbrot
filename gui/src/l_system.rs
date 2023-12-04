use std::collections::HashMap;
use std::ops::{Add, AddAssign};
use std::str::Chars;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct LStr(String);

impl LStr {
    pub(crate) fn new() -> Self {
        Self::from("")
    }
}

impl LStr {
    pub(crate) fn chars(&self) -> Chars<'_> {
        self.0.chars()
    }
}

impl From<&str> for LStr {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl From<String> for LStr {
    fn from(value: String) -> Self {
        LStr(value)
    }
}

impl AddAssign for LStr {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += &rhs.0;
    }
}

impl AddAssign<String> for LStr {
    fn add_assign(&mut self, rhs: String) {
        *self += LStr::from(rhs);
    }
}

pub struct LSystem<'a> {
    // turtle: &'a mut Turtle<'a>,
    start: &'a LStr,
    rules: &'a HashMap<char, LStr>,
}

impl<'a> LSystem<'a> {
    pub fn new(start: &'a LStr, rules: &'a HashMap<char, LStr>) ->
    Self {
        Self {
            start,
            rules,
        }
    }

    pub fn iter(&'a self) -> LSystemIter<'a> {
        LSystemIter::new(self)
    }
}

pub struct LSystemIter<'a> {
    l_system: &'a LSystem<'a>,
    current: LStr,
    n: usize,
}

impl<'a> LSystemIter<'a> {
    pub fn new(l_system: &'a LSystem<'a>) -> Self {
        Self {
            l_system,
            current: l_system.start.clone(),
            n: 0,
        }
    }

    pub fn n(&self) -> usize {
        self.n
    }
}

impl<'a> Iterator for LSystemIter<'a> {
    type Item = LStr;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            self.n += 1;
            return Some(self.current.clone());
        }
        let mut next = String::new();
        let mut len = 0;
        for c in self.current.chars() {
            if let Some(rule) = self.l_system.rules.get(&c) {
                next.push_str(&rule.0);
            } else {
                next.push(c);
            }
            len += 1;
        }

        let r = next.chars().skip(len).collect::<String>().into();
        self.current = next.into();
        self.n += 1;

        Some(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l_system_iter() {
        let start = LStr::from("A");
        let rules = HashMap::from(
            [
                ('A', LStr::from("AB")),
                ('B', LStr::from("A"))
            ],
        );
        let l_system = LSystem::new(&start, &rules);

        let mut iter = l_system.iter();

        assert_eq!(iter.next(), Some(LStr::from("AB")));
        assert_eq!(iter.next(), Some(LStr::from("ABA")));
        assert_eq!(iter.next(), Some(LStr::from("ABAAB")));
        assert_eq!(iter.next(), Some(LStr::from("ABAABABA")));
    }
}

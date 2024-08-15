// struct CombinationIter<T> {
//     iterable: Vec<T>,
//     r: usize,
// }

use std::vec;

pub struct CombinationIter<'a, T> {
    iterable: &'a Vec<T>,
    r: usize,
    n: usize,
}

impl<'a, T> CombinationIter<'a, T> {
    pub fn new(iterable: &'a Vec<T>, a: usize) -> Self {
        Self {
            iterable: iterable,
            r: a,
            n: 0,
        }
    }
}

impl<'a, T: 'a> Iterator for CombinationIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        if self.n - 1 < self.iterable.len() {
            return Some(&self.iterable[self.n - 1]);
        }
        return None;
    }
}

pub fn combinations<T>(elements: Vec<T>, r: usize) -> Vec<Vec<T>>
where
    T: Clone + Copy,
{
    if r == 0 {
        return vec![vec![]];
    }
    if elements.len() < r {
        return vec![];
    }
    let mut with_first = combinations(elements[1..].to_vec(), r - 1);
    let first_element = vec![elements[0]];
    with_first = with_first
        .iter()
        .map(|a| [first_element.clone(), a.clone()].concat())
        .collect();
    let without_first = combinations(elements[1..].to_vec(), r);
    [with_first, without_first].concat()
}

pub fn insert_space(s: Vec<&str>, n: usize) -> Vec<String> {
    let position: Vec<usize> = (0..(s.len() + 1)).collect();
    let mut rlist = Vec::new();

    for i in combinations(position, n) {
        let mut new_str = vec![];
        let mut last_index: usize = 0;
        for index in i {
            new_str.push(s[last_index..index].join(""));
            new_str.push(" ".to_string());
            last_index = index;
        }
        new_str.push(s[last_index..].join(""));
        rlist.push(new_str.join(""));
    }
    rlist
}

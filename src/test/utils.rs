// struct CombinationIter<T> {
//     iterable: Vec<T>,
//     r: usize,
// }

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

pub fn combinations<'a>(elements: Vec<&'a str>, r: usize) -> Vec<Vec<&'a str>> {
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
    return [with_first, without_first].concat();
}

// fn func() {
//     let mut a = vec!["hello", "world", "Tom"];
//     let iter_example = CombinationIter::new(&a, 2);
//     for i in iter_example {
//         println!("{}", i)
//     }
// }

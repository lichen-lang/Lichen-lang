/// # combinations
/// function for test
/// ```rust
/// let a = vec!["a","b","c", "d", "e"];
/// let b = combinations(a, 3);
/// println!("{:?}", b);
/// // [["a", "b", "c"], ["a", "b", "d"], ["a", "b", "e"], ["a", "c", "d"], ["a", "c", "e"], ["a", "d", "e"], ["b", "c", "d"], ["b", "c", "e"], ["b", "d", "e"], ["c", "d", "e"]]
/// ```
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

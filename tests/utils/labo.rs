#[derive(Debug)]
struct ListBlockBranchEx<T>
where
    T: TokenEx,
{
    code_list: Vec<T>,
}

trait TokenEx {}

#[derive(Debug)]
enum ExprElem {
    A,
    B,
    C,
}

#[derive(Debug)]
enum TypeElem {
    A,
    B,
    C,
}

impl TokenEx for ExprElem {}
impl TokenEx for TypeElem {}

impl<T> ListBlockBranchEx<T>
where
    T: TokenEx,
{
    fn new(code_list: Vec<T>) -> Self {
        Self { code_list }
    }
}

fn function00(lbb: ListBlockBranchEx<TypeElem>) {}

#[cfg(test)]
mod tests {
    use super::ExprElem;
    use super::ListBlockBranchEx;
    use super::TypeElem;

    #[test]
    fn test00() {
        let a = ListBlockBranchEx::new(vec![ExprElem::A, ExprElem::B]);
        let b = ListBlockBranchEx::new(vec![TypeElem::A, TypeElem::B]);
        println!("{:?}", a);
        println!("{:?}", b);
    }
}

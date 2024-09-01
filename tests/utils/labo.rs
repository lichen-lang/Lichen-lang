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

fn function01<T>(lbb: &ListBlockBranchEx<T>) -> &T
where
    T: TokenEx,
{
    &lbb.code_list[0]
}

#[cfg(test)]
mod tests {
    use crate::utils::labo::function01;

    use super::ExprElem;
    use super::ListBlockBranchEx;
    use super::TypeElem;

    #[test]
    fn test00() {
        let a = ListBlockBranchEx::new(vec![ExprElem::A, ExprElem::B, ExprElem::C]);
        let b = ListBlockBranchEx::new(vec![TypeElem::A, TypeElem::B, TypeElem::C]);
        println!("{:?}", a);
        println!("{:?}", b);
    }

    #[test]
    fn test01() {
        let a = ListBlockBranchEx::new(vec![ExprElem::A, ExprElem::B, ExprElem::C]);
        let b = function01(&a);
        println!("{:?}", b);
        println!("{:?}", a);
    }
}

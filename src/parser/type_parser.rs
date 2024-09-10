use crate::errors::parser_errors::ParserError;

use crate::abs::ast::*;

pub struct TypeParser {
    pub code: String,
    pub code_list: Vec<TypeElem>,
    pub depth: isize,
    pub loopdepth: isize,
}

impl TypeParser {
    fn code2vec(&mut self) -> Result<(), ParserError> {
        self.grouping_elements(TypeElem::TypeBlockElem, '<', '>')?;
        Ok(())
    }

    fn grouping_elements<T>(
        &mut self,
        elemtype: fn(T) -> TypeElem,
        open_char: char,
        close_char: char,
    ) -> Result<(), ParserError>
    where
        T: TypeAreaBranch,
    {
        let mut rlist: Vec<TypeElem> = Vec::new();
        let mut group: Vec<TypeElem> = Vec::new();
        let mut depth: isize = 0;

        for inner in &self.code_list {
            if let TypeElem::UnKnownElem(ref b) = inner {
                if b.contents == open_char {
                    match depth {
                        0 => { /*pass*/ }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                    depth += 1;
                } else if b.contents == close_char {
                    depth -= 1;
                    match depth {
                        0 => {
                            rlist.push(elemtype(TypeAreaBranch::new(group.clone(), self.depth)));
                            group.clear();
                        }
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                } else {
                    match depth {
                        0 => rlist.push(inner.clone()),
                        1.. => group.push(inner.clone()),
                        _ => return Err(ParserError::BraceNotOpened),
                    }
                }
            } else {
                match depth {
                    0 => rlist.push(inner.clone()),
                    1.. => group.push(inner.clone()),
                    _ => return Err(ParserError::BraceNotClosed),
                }
            }
        }
        if depth != 0 {
            return Err(ParserError::BraceNotClosed);
        }
        self.code_list = rlist;
        Ok(())
    }
}

# token modules

- recursion parse token (that implement RecursiveAnalysisElements)

  each module has `resolve_self` method and it parse itself inner contents 

  - block

  - func

  - list_block

  - paren_block

  - syntax_box

  - syntax

# Rule of `ExprElem`

after parsing, there are two possible states of `Vec<ExprElem>`

- length of `Vec<ExprElem>` 1

  this is the case that `ExprElem` is pure expression, or only item of the list that has a one element

- length of `Vec<ExprElem>` 1 <
  
  `ExprElem` is item of list, or args of function 
  
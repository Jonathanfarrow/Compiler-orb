# Language Grammar

| Rule | Definition |
|------|------------|
| expression | `logical_or` |
| logical_or | `logical_and ( "||" logical_and )*` |
| logical_and | `equality ( "&&" equality )*` |
| equality | `comparison ( ( "==" \| "!=" ) comparison )*` |
| comparison | `term ( ( "<" \| "<=" \| ">" \| ">=" ) term )*` |
| term | `factor ( ( "+" \| "-" ) factor )*` |
| factor | `unary ( ( "*" \| "/" \| "%" ) unary )*` |
| unary | `( "!" \| "-" \| "+" ) unary \| tensor_op \| trig_function` |
| tensor_op | `primary ( tensor_operator primary )*` |
| primary | `NUMBER \| STRING \| "true" \| "false" \| "nil"` <br> `\| "(" expression ")"` <br> `\| IDENTIFIER` <br> `\| tensor_literal` <br> `\| matrix_literal` <br> `\| vector_literal` <br> `\| function_call` <br> `\| trig_function` |
| tensor_operator | `"@" \| "⊗" \| "•" \| "×"` |
| function_call | `IDENTIFIER "(" ( expression ( "," expression )* )? ")"` |
| tensor_literal | `"[" ( tensor_row ( ";" tensor_row )* )? "]"` |
| tensor_row | `"[" ( expression ( "," expression )* )? "]"` |
| matrix_literal | `"[" ( vector_literal ( ";" vector_literal )* )? "]"` |
| vector_literal | `"[" ( expression ( "," expression )* )? "]"` |
| trig_function | `"sin" "(" expression ")"` <br> `\| "cos" "(" expression ")"` <br> `\| "tan" "(" expression ")"` <br> `\| "asin" "(" expression ")"` <br> `\| "acos" "(" expression ")"` <br> `\| "atan" "(" expression ")"` <br> `\| "sinh" "(" expression ")"` <br> `\| "cosh" "(" expression ")"` <br> `\| "tanh" "(" expression ")"` <br> `\| "log" "(" expression ")"` <br> `\| "exp" "(" expression ")"` <br> `\| "sqrt" "(" expression ")"` |

Note: `*` denotes zero or more occurrences, `?` denotes optional elements.

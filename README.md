expression → logical_or
logical_or → logical_and ( "||" logical_and )*
logical_and → equality ( "&&" equality )*
equality → comparison ( ( "==" | "!=" ) comparison )*
comparison → term ( ( "<" | "<=" | ">" | ">=" ) term )*
term → factor ( ( "+" | "-" ) factor )*
factor → unary ( ( "*" | "/" | "%" ) unary )*
unary → ( "!" | "-" | "+" ) unary | tensor_op | trig_function
tensor_op → primary ( tensor_operator primary )*
primary → NUMBER | STRING | "true" | "false" | "nil" 
        | "(" expression ")" 
        | IDENTIFIER
        | tensor_literal
        | matrix_literal
        | vector_literal
        | function_call
        | trig_function

tensor_operator → "@" | "⊗" | "•" | "×"  // Example tensor operators
function_call → IDENTIFIER "(" ( expression ( "," expression )* )? ")"
tensor_literal → "[" ( tensor_row ( ";" tensor_row )* )? "]"
tensor_row → "[" ( expression ( "," expression )* )? "]"
matrix_literal → "[" ( vector_literal ( ";" vector_literal )* )? "]"
vector_literal → "[" ( expression ( "," expression )* )? "]"

trig_function → "sin" "(" expression ")" 
              | "cos" "(" expression ")" 
              | "tan" "(" expression ")" 
              | "asin" "(" expression ")" 
              | "acos" "(" expression ")" 
              | "atan" "(" expression ")" 
              | "sinh" "(" expression ")" 
              | "cosh" "(" expression ")" 
              | "tanh" "(" expression ")" 
              | "log" "(" expression ")" 
              | "exp" "(" expression ")" 
              | "sqrt" "(" expression ")" 

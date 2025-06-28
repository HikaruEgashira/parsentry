(call_expression
  function: (identifier) @function_call)

(call_expression
  function: (field_expression
    field: (field_identifier) @function_call))

(call_expression
  function: (scoped_identifier
    name: (identifier) @function_call))

(macro_invocation
  macro: (identifier) @function_call)

; Method calls
(method_call_expression
  method: (field_identifier) @function_call)

; Function calls with generic parameters
(call_expression
  function: (generic_function
    function: (identifier) @function_call))

; Associated function calls
(call_expression
  function: (scoped_identifier
    path: (identifier)
    name: (identifier) @function_call))

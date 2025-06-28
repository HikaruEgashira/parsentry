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
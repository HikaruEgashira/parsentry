; Direct function calls
(call_expression
  function: (identifier) @direct_call)

; Function pointer calls
(call_expression
  function: (field_expression
    field: (field_identifier) @method_call))

; Function references (assignment)
(assignment_expression
  left: (identifier)
  right: (identifier) @reference)

; Function as parameter (callbacks)
(parameter_list
  (parameter_declaration
    declarator: (identifier) @callback))

; Function in argument list (callbacks)
(argument_list
  (identifier) @callback)

; Include statements
(preproc_include
  path: (string_literal) @import)
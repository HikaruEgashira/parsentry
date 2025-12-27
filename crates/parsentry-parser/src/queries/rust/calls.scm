; Direct function calls
(call_expression
  function: (identifier) @direct_call)

; Method calls via field access
(call_expression
  function: (field_expression
    field: (field_identifier) @method_call))

; Scoped function calls
(call_expression
  function: (scoped_identifier
    name: (identifier) @direct_call))

; Macro calls
(macro_invocation
  macro: (identifier) @macro_call)

; Function calls with generic parameters
(call_expression
  function: (generic_function
    function: (identifier) @direct_call))

; Function references (not calls)
(let_declaration
  value: (identifier) @reference)

; Function as argument
(arguments
  (identifier) @callback)

; Use declarations
(use_declaration
  (scoped_identifier) @import)

; Assignment
(assignment_expression
  right: (identifier) @assignment)
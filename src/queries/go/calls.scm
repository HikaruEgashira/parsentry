; Direct function calls
(call_expression
  function: (identifier) @direct_call)

; Method calls
(call_expression
  function: (selector_expression
    field: (field_identifier) @method_call))

; Function references
(var_spec
  name: (identifier)
  value: (identifier) @reference)

; Function as argument (callbacks)
(argument_list
  (identifier) @callback)

; Import statements
(import_spec
  path: (interpreted_string_literal) @import)

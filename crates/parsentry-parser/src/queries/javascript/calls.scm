; Direct function calls
(call_expression
  function: (identifier) @direct_call)

(call_expression
  function: (member_expression
    property: (property_identifier) @method_call))

; Function references
(variable_declarator
  value: (identifier) @reference)

; Function as argument (callbacks)
(arguments
  (identifier) @callback)

; Import statements
(import_statement
  source: (string) @import)

; Assignment expressions
(assignment_expression
  left: (identifier)
  right: (identifier) @assignment)
; Direct function calls
(call_expression
  function: (identifier) @direct_call)

; Method calls
(call_expression
  function: (member_expression
    property: (property_identifier) @method_call))

; Computed method calls
(call_expression
  function: (member_expression
    property: (computed_property_name) @method_call))

; Function references (assignment)
(variable_declarator
  value: (identifier) @reference)

; Assignment expressions
(assignment_expression
  left: (identifier)
  right: (identifier) @assignment)

; Function as argument (callbacks)
(arguments
  (identifier) @callback)

; Arrow functions as arguments (callbacks)
(arguments
  (arrow_function) @callback)

; Import statements
(import_statement
  source: (string) @import)

; Type imports
(import_statement
  (import_clause) @import)
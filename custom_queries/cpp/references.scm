(call_expression
  function: (identifier) @reference)

(call_expression
  function: (field_expression
    field: (field_identifier) @reference))

(call_expression
  function: (qualified_identifier
    name: (identifier) @reference))

(identifier) @reference
(call_expression
  function: (identifier) @reference)

(call_expression
  function: (qualified_identifier
    name: (identifier) @reference))

(call_expression
  function: (field_expression
    field: (field_identifier) @reference))

(identifier) @reference

(type_identifier) @reference

(qualified_identifier
  name: (identifier) @reference)
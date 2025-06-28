(call_expression
  function: (identifier) @function_call)

(call_expression
  function: (member_expression
    property: (property_identifier) @function_call))

(call_expression
  function: (member_expression
    property: (computed_property_name) @function_call))
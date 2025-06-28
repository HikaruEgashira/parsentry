; Direct function calls
(function_call_expression
  function: (name) @direct_call)

; Method calls
(member_call_expression
  name: (name) @method_call)

; Function references (assignment)
(assignment_expression
  left: (variable_name)
  right: (name) @reference)

; Function as argument (callbacks)
(argument_list
  (name) @callback)
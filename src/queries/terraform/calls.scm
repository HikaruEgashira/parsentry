; Function calls
(function_call
  function_name: (identifier) @direct_call)

; Variable references
(variable_expr
  (identifier) @reference)

; Local value references
(variable_expr
  (identifier) @reference)

; Module calls
(module_call
  (identifier) @method_call)

; Data source references
(variable_expr
  (identifier) @reference)

; Resource references
(variable_expr
  (identifier) @reference)
; Variable references
(variable_expr
  (identifier) @variable.reference)

; Local value references  
(local_value
  (identifier) @local.reference)

; Resource attribute references
(get_attr
  (expression) @resource.reference
  (identifier) @attribute.reference)

; Module output references
(index_expr
  (expression) @module.reference
  (string_literal) @output.reference)

; Data source references
(get_attr
  (get_attr
    (identifier) @data.type
    (identifier) @data.name)
  (identifier) @data.attribute)

; Function call references
(function_call
  name: (identifier) @function.reference
  arguments: (arguments) @function.arguments)

; Interpolation expressions
(template_expr
  (template_interpolation
    (expression) @interpolation.expr))

; For expressions
(for_expr
  key: (identifier)? @for.key
  value: (identifier) @for.value
  collection: (expression) @for.collection
  condition: (expression)? @for.condition)

; Conditional expressions
(conditional_expr
  condition: (expression) @conditional.condition
  true_expr: (expression) @conditional.true
  false_expr: (expression) @conditional.false)

; Object attribute access
(get_attr
  object: (expression) @object.reference
  attr: (identifier) @object.attribute)

; Splat expressions
(splat_expr
  (expression) @splat.object
  (identifier) @splat.attribute)
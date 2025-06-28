; Direct method calls
(method_invocation
  name: (identifier) @direct_call)

; Method calls with object
(method_invocation
  object: (_)
  name: (identifier) @method_call)

; Constructor calls
(object_creation_expression
  type: (type_identifier) @direct_call)

; Function references (assignment)
(assignment_expression
  left: (identifier)
  right: (identifier) @reference)

; Lambda expressions as arguments (callbacks)
(argument_list
  (lambda_expression) @callback)

; Method references as arguments (callbacks)
(argument_list
  (method_reference) @callback)

; Import statements
(import_declaration
  (scoped_identifier) @import)
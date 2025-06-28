; Direct function calls
(call
  function: (identifier) @direct_call)

(call
  function: (attribute
    attribute: (identifier) @method_call))

; Function references
(assignment
  value: (identifier) @reference)

; Function as argument (callbacks)
(argument_list
  (identifier) @callback)

; Import statements
(import_statement
  name: (dotted_name) @import)

(import_from_statement
  name: (dotted_name) @import)
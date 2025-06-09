; Variable references
(variable_name) @reference

; Function calls
(function_call_expression
  function: (name) @reference)

; Method calls
(member_call_expression
  name: (name) @reference)

; Static method calls
(scoped_call_expression
  name: (name) @reference)

; Property access
(member_access_expression
  name: (name) @reference)

; Static property access
(scoped_property_access_expression
  name: (name) @reference)

; Class name references
(named_type
  (name) @reference)

; Qualified names (namespaced references)
(qualified_name
  (name) @reference)

; Use statements
(namespace_use_declaration
  (namespace_use_clause
    (qualified_name) @reference))

; Trait use
(use_declaration
  (name) @reference)

; Constant references
(const_element
  value: (name) @reference)

; Array access
(subscript_expression
  (name) @reference)

; Object creation
(object_creation_expression
  (name) @reference)

; instanceof checks
(instanceof_expression
  right: (name) @reference)

; Global variable references
(global_declaration
  (variable_name) @reference)

; Include/require statements - these often reference files
(include_expression) @reference
(include_once_expression) @reference
(require_expression) @reference
(require_once_expression) @reference
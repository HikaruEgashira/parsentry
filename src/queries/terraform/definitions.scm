; Block definitions with type and name
(block
  (identifier) @block.type
  (string_lit)? @name
  (body) @definition) @definition

; Function calls
(function_call
  (identifier) @name) @definition

; Attribute definitions
(attribute
  (identifier) @name
  (expression) @definition) @definition
; Function definitions
(function_definition
  name: (name) @name) @definition

; Method declarations
(method_declaration
  name: (name) @name) @definition

; Class declarations
(class_declaration
  name: (name) @name) @definition

; Interface declarations
(interface_declaration
  name: (name) @name) @definition

; Trait declarations
(trait_declaration
  name: (name) @name) @definition

; Enum declarations
(enum_declaration
  name: (name) @name) @definition

; Anonymous function expressions (closures)
(anonymous_function
  parameters: (formal_parameters) @name) @definition

; Arrow functions
(arrow_function
  parameters: (_) @name) @definition

; Property declarations
(property_declaration
  (property_element
    (variable_name) @name)) @definition

; Const declarations
(const_declaration
  (const_element
    name: (name) @name)) @definition

; Function-like method names (constructors, destructors)
(method_declaration
  name: (name) @name
  (#match? @name "^(__construct|__destruct|__call|__callStatic|__get|__set|__isset|__unset|__sleep|__wakeup|__toString|__invoke|__set_state|__clone|__debugInfo)$")) @definition
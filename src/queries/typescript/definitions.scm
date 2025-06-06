(function_declaration
  name: (identifier) @name) @definition

(generator_function_declaration
  name: (identifier) @name) @definition

(class_declaration
  name: (type_identifier) @name) @definition

(method_definition
  name: (property_identifier) @name) @definition

(interface_declaration
  name: (type_identifier) @name) @definition

(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(arrow_function)]
  ) @definition
)

(variable_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(arrow_function)]
  ) @definition
)

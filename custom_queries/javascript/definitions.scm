(function_declaration
  name: (identifier) @name) @definition

(generator_function_declaration
  name: (identifier) @name) @definition

(class_declaration
  name: (identifier) @name) @definition

(method_definition
  name: (property_identifier) @name) @definition

(variable_declarator
  name: (identifier) @name
  value: [(function_expression) (arrow_function)]
) @definition

(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    value: [(function_expression) (arrow_function)]
  ) @definition
)

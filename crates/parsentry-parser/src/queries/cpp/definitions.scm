(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition

(function_definition
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition

(function_definition
  declarator: (function_declarator
    declarator: (qualified_identifier
      name: (identifier) @name))) @definition

(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition

(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition

(declaration
  declarator: (function_declarator
    declarator: (field_identifier) @name)) @definition

(declaration
  declarator: (function_declarator
    declarator: (qualified_identifier
      name: (identifier) @name))) @definition

(declaration
  declarator: (init_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition

(class_specifier
  name: (type_identifier) @name) @definition

(struct_specifier
  name: (type_identifier) @name) @definition

(enum_specifier
  name: (type_identifier) @name) @definition

(type_definition
  declarator: (type_identifier) @name) @definition

(namespace_definition
  name: (namespace_identifier) @name) @definition

(preproc_function_def
  name: (identifier) @name) @definition
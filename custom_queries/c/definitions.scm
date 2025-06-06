(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition

(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition

(declaration
  declarator: (function_declarator
    declarator: (identifier) @name)) @definition

(declaration
  declarator: (init_declarator
    declarator: (function_declarator
      declarator: (identifier) @name))) @definition

(struct_specifier
  name: (type_identifier) @name) @definition

(enum_specifier
  name: (type_identifier) @name) @definition

(type_definition
  declarator: (type_identifier) @name) @definition

(preproc_function_def
  name: (identifier) @name) @definition
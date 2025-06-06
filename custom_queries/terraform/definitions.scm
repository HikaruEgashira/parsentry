; Resource definitions
(resource 
  type: (identifier) @resource.type
  name: (string_literal) @resource.name
  body: (body) @resource.body)

; Data source definitions  
(data
  type: (identifier) @data.type
  name: (string_literal) @data.name
  body: (body) @data.body)

; Variable definitions
(variable
  name: (string_literal) @variable.name
  body: (body) @variable.body)

; Output definitions
(output
  name: (string_literal) @output.name  
  body: (body) @output.body)

; Module definitions
(module
  name: (string_literal) @module.name
  body: (body) @module.body)

; Provider definitions
(provider
  name: (identifier) @provider.name
  body: (body) @provider.body)

; Local value definitions
(locals
  body: (body) @locals.body)

; Terraform configuration blocks
(terraform
  body: (body) @terraform.body)

; Function calls
(function_call
  name: (identifier) @function.name
  arguments: (arguments) @function.args)

; Attribute assignments
(attribute
  name: (identifier) @attribute.name
  value: (_) @attribute.value)

; Block definitions
(block
  type: (identifier) @block.type
  labels: (string_literal)* @block.labels  
  body: (body) @block.body)
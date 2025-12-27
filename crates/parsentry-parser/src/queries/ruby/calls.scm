; Direct method calls
(call
  method: (identifier) @direct_call)

; Method calls with receiver
(call
  receiver: (_)
  method: (identifier) @method_call)

; Method references (assignment)
(assignment
  left: (identifier)
  right: (identifier) @reference)

; Block as argument (callbacks)
(call
  block: (block) @callback)

; Proc as argument (callbacks)
(argument_list
  (call
    method: (identifier) @callback))

; Require statements
(call
  method: (identifier) @import
  arguments: (argument_list
    (string) @import))

; Include/extend statements  
(call
  method: (identifier) @import
  arguments: (argument_list
    (constant) @import))
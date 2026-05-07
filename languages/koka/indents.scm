[
  (opexpr [index: (arguments) call: (arguments)])
  (atom ["[" "("])
  (funbody)
  (block)
  (constructor)
  (handlerexpr)
  (opclausex)
] @indent

[
  (typedecl
    [(typeid) (opdecls)])
  (externdecl)
  (matchexpr)
  (matchrule)
  "then"
  "else"
] @indent @extend

(matchrule "->" @indent @extend)

(ERROR "fun") @indent @extend
(ERROR "match") @indent @extend
(ERROR "->" @indent.always @extend)

(atom ")" @outdent @extend.prevent-once)

[
  "]"
  "}"
] @outdent @extend.prevent-once

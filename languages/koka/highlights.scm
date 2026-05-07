; Comments

[
  (linecomment)
  (blockcomment)
] @comment

; Literals

(string) @string
(char) @string
(escape) @string.escape

(float) @number
(int) @number

; Delimiters

(matchrule "|" @punctuation.delimiter)
(tatomic "|" @punctuation.delimiter)

[
  ","
  "->"
  "."
  ":"
  "::"
  "<-"
  ";"
] @punctuation.delimiter

[
  "<"
  ">"
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

; Keywords

[
  "as"
  (externtarget)
  "forall"
  "handle"
  "handler"
  "in"
  "infix"
  "infixl"
  "infixr"
  "mask"
  (behindmod)
  (pub)
  "some"
] @keyword

(constructor "lazy" @keyword)
(matchexpr "lazy" @keyword)

[
  (con)
  "ctl"
  "fn"
  "fun"
] @keyword

"with" @keyword

[
  "elif"
  "else"
  "if"
  "match"
  "then"
] @keyword

[
  "import"
  "module"
] @keyword

[
  "alias"
  "effect"
  "struct"
  "type"
  "val"
  "var"
] @keyword

[
  "abstract"
  "extern"
  "final"
  (inlinemod)
  (externinline)
  (typemod)
  (structmod)
  (effectmod)
  "named"
  (override)
  (controlmod)
  (tailmod)
] @keyword

(fipmod ["fip" "fbip"] @keyword)

"return" @keyword

; Operators

[
  "!"
  "~"
  "="
  ":="
  (idop)
  (op)
  (qidop)
] @operator

(modulepath) @namespace

; Variables

(pattern
  (identifier
    (varid) @variable))

(paramid
  (identifier
    (varid) @variable.parameter))

(pparameter
  (pattern
    (identifier
      (varid) @variable.parameter)))

(pparameter
  (qimplicit) @variable.parameter)

(puredecl
  (binder
    (qidentifier) @constant))

(argument
  [(identifier) (qimplicit)] @variable.parameter
  "="
  (expr))

; Types

(typecon
  [(varid) (qvarid)] @type)

(tbinder
  (varid) @type)

(typeid
  (varid) @type)

(typedecl
  "effect"
  (varid) @type)

; Function definitions

(fundecl
  (identifier) @function)

(puredecl
  (qidentifier) @function)

(externdecl
  (qidentifier) @function)

(opclause
  (qidentifier) @function)

(operation
  (identifier) @function)

; Function calls

(opexpr
  (atom
    (name) @function)
  .
  [
    call: "(" (arguments)? ")"
    trailing_lambda: [(block) (fnexpr)]
  ])

(opexpr
  (atom)
  (name) @function)

(ntlexpr
  (atom
    (name) @function)
  .
  ("(" (arguments)? ")"))

(ntlexpr
  (atom)
  (name) @function)

[(conid) (qconid)] @constructor

[
  "initially"
  "finally"
] @function

# dead simple relaxed json

usage: `let j: String = dsrj::compile(&string)`.
dsrj never fails, UB for invalid input.

a preprocessor compile dsrj to json. dsrj is json but:

* allow hash comment, but only at non-blank begin of line

* allow trailing comma, but only at the end of line,
so the first non-blank char of next line is `]` or `}`

example:

```
{
	# example dsrj
	"a": "b",
}
```

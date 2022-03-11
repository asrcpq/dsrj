# dead simple relaxed json

a preprocessor(lib and bin) with zero dependency

json but:

* allow hash comment, but only at non-blank begin of line

* allow trailing comma, but only at end of line
(the first non-blank char of next line is `]` or `}`)

example:

```
{
	// example dsrj
	"a": "b",
}
```

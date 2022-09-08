# Dead simple relaxed json

Because I need a dead simple configuration language.

A preprocessor compiles dsrj to json: dsrj is json but:

* allow hash comment, but only at non-blank begin of line

* allow trailing comma, but only at the end of line,
so the first non-blank char of next non-blank, non-comment line is `]` or `}`

example: see `/example.rjson`

## Notes

* This repository is referred as the definition of Dsrj,
write your own implementations.

* Dsrj files don't have a standard extension,
I personally use '.rjson'

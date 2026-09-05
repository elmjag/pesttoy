# simple PEG parser

A simple PEG parser for a toy language using rust crate `pest`.
The language syntax is specified using flavor of parsing expression grammar (PEG),
as supported by `pest` crate.

# language

Here is an example of the toy language implemented.

    x <- 10
    y <- 5 - 1
    z <- (1 + 2) * 3
    x + y * 10 + z

The language support assignment of expressions to variables.
Each assignment statments is separed by a newline (`\n`).
Last line must be an expression, which is the result of the program.

## grammar

See [toy.pest](src/toy.pest) for language's grammar specification.

# usage

    $ toy source-file0 source-file1 ...

The `toy` binary will parse and evaluate each specified source file.
The result will be printed to stdout.
Note that error handling is very primitive,
`toy` will panic on invalid syntax.

# building

Make sure you have the standard Rust tooling installed.
Build the binary with:

    cargo build

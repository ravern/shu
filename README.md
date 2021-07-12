# Oma

Practical gradually-typed programming language.

## Crates

* `oma` - Stack-based virtual machine.
* `oma-bootstrap` - Bootstrap compiler for Oma into various targets (e.g. JavaScript, VM bytecode, WASM). Goal is to transition to one built in Oma.
* `oma-cli` - Command-line interface to virtual machine and compiler.
* `shu` - Package manager.

















Why do we preserve spans? So that when there's an error, we can report it nicely. Spans don't have to be on anything that won't error.


AST - Representation of source code, enough to give accurate errors.
IR - Resolved syntax tree; Resolution of modules, locals, fn calls etc. all completed

source |> parser.parse       // errors related to invalid syntax; collect all and continue using syncs
ast    |> resolver.resolve   // errors related to undefined variables etc.; collect all
ir     |> generator.generate // no error

# Oma

Practical Lisp-based programming language.

## Usage

```bash
oma index.oma

shu add dash@1.2.3
shu remove blast
shu install
```

## Guide

Basic forms.

```lisp
(def variable value-of-variable)

(if condition if-body else-body)

(lambda (arg-one arg-two) body)

'(this is a quoted list)
`(this is a quasi-quoted list with ,unquoting)

(cons head tail)
(head list)
(tail list)
```

Define a function.

```lisp
(defn multiply (foo bar)
  (* foo bar))
```

Define a macro.

```lisp
(defmcr add (foo bar)
  `(* ,foo ,bar))
```

## Development

Oma uses an incremental compilation strategy to improve performance while maintaining Lisp's dynamic and homoiconic nature.

```rust
let machine = Machine::new();
machine.compile_file("index.oma")
machine.call("main");
```

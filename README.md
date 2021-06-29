# Oma

Practical Lisp-based programming language.

## Usage

```bash
oma run index.oma
oma compile index.oma
```

## Guide

Basic forms.

```lisp
(def variable value-of-variable)

(if condition if-body else-body)

(lambda (arg-one arg-two) body)
```

Define a function.

```lisp
(defn foo (bar baz)
  (* bar baz))
```

Define a macro.

```
(defmcr foo 
```

## Development

Oma uses an incremental compilation

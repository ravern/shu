use crate::value::Lambda;

mod parse;

// 1. Parse the file as-is.
// 2. Compile the file as-is (without evaluating any
//    macros or what-not). This should produce the
//    "undefined symbol" errors when calling `fn` and
//    `mod`.


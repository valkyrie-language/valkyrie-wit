Currently, `wasmtime`'s error in wat text files are based on the compilation offset.

When an error occurs, it is very confusing where the error occurred.

I would like to attach some source language location information to point out the error location.

Similar to:

```wat
;; @url: file://path/position.wasm:line:column
(type $test-type (externref))
;; @url: file://path/position.wasm:offset
(func $test-function
    ;; @file: main.rs:12:24
    (block $test-block

    )
)
```

`binaryen` supports similar features
like: https://github.com/WebAssembly/binaryen/blob/921644ca65afbafb84fb82d58dacc4a028e2d720/test/lit/debug/full.wat#L49-L64


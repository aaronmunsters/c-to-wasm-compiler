# C to WebAssembly Compiler

Rust crate for compiling C source code to WebAssembly using [Emscripten](https://emscripten.org/).

## Prerequisites

Requires [Emscripten](https://emscripten.org/) installed and in your PATH.

## Usage

```rust
use c_to_wasm_compiler::{Compiler, configuration_builder::ConfigurationBuilder};
use c_to_wasm_compiler::configuration::{Debugging, Profile};

let c_source = r#"
    #include <stdint.h>
    
    __attribute__((export_name("add")))
    int32_t add(int32_t a, int32_t b) {
        return a + b;
    }
"#;

let config = ConfigurationBuilder::init()
    .source(c_source.into())
    .profile(Profile::O2)
    .debugging(Debugging::Disabled)
    .build();

let wasm_bytes = Compiler::compile(&config)?;
```

## Configuration

**Profiles**: `O0`, `O1`, `O2`, `O3`  
**Debugging**: `Enabled`, `Disabled`

## Exporting C Functions

Use the [Emscripten](https://emscripten.org/) export attribute to make functions callable from WASM:

```c
__attribute__((export_name("my_function")))
int32_t my_function(int32_t arg) {
    return arg * 2;
}
```

## Author

Aäron Munsters

## License

MIT

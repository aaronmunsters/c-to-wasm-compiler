use std::fs::File;
use std::io::{BufReader, Read, Write};

use tempfile::NamedTempFile;

pub mod configuration;
pub mod configuration_builder;
pub mod error;

use configuration::Configuration;
use error::Error;

#[derive(Default, Clone, Copy, Debug)]
pub struct Compiler;

impl Compiler {
    /// Compiles the current configuration into a WebAssembly module using
    /// the [emscripten compiler](https://emscripten.org/).
    ///
    /// # Errors
    /// - If using the host's file system fails.
    /// - If compilation fails
    pub fn compile(&self, configuration: &Configuration) -> Result<Vec<u8>, Error> {
        let input_source = NamedTempFile::with_suffix("_c-to-wasm-source.c").map_err(Error::IO)?;
        let output_wasm = NamedTempFile::with_suffix("_c-to-wasm-out.wasm").map_err(Error::IO)?;

        // Write into temp file
        input_source
            .as_file()
            .write_all(configuration.source().as_bytes())
            .map_err(Error::IO)?;

        let mut command = configuration.as_command(input_source.path(), output_wasm.path());
        let status = command.status().map_err(Error::IO)?;
        let output = command.output().map_err(Error::IO)?;

        if !status.success() {
            return Err(Error::Unsuccesful(output));
        }

        // Read from temp file
        let output_file = File::open(output_wasm.path()).map_err(Error::IO)?;
        let mut reader = BufReader::new(output_file);
        let mut output_content = vec![];
        reader.read_to_end(&mut output_content).map_err(Error::IO)?;

        Ok(output_content)
    }
}

use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::process::Command;

/* re-export the semver version */
pub use semver::Version;

use ctreg::regex;
use tempfile::NamedTempFile;

pub mod configuration;
pub mod configuration_builder;
pub mod error;

use configuration::Configuration;
use error::{Error, VersionError};

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
        let output = command.output().map_err(Error::IO)?;

        if !output.status.success() {
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

regex! { pub EmccSemVerRegex = r"emcc \(Emscripten gcc\/clang-like replacement \+ linker emulating GNU ld\) (?<semver>\d*\.\d*\.\d*) \([\d\w]*\)" }

impl Compiler {
    /// Yields the version of the [emscripten compiler](https://emscripten.org/) as
    /// a semver struct.
    ///
    /// # Errors
    /// - If [emscripten](https://emscripten.org/) is not installed on the host
    /// - If the version cannot be read from the command output
    pub fn version() -> Result<Version, VersionError> {
        // Invoke command to request emcc version
        let output = Command::new("emcc")
            .arg("--version")
            .output()
            .map_err(VersionError::IO)?;

        // If invocation failed, yield early
        if !output.status.success() {
            return Err(VersionError::InvocationNoSuccess(output));
        }

        // Parse command output to `String`
        let command_output =
            String::try_from(output.stdout).map_err(VersionError::AttemptReadStdOut)?;

        // Parse command ourput to matching semver specification regex
        let Some(semver) = EmccSemVerRegex::new().captures(&command_output) else {
            return Err(VersionError::RegexNoMatch(command_output));
        };

        // Parse matching regex specification to `Semver`
        Version::parse(semver.semver.content).map_err(VersionError::VersionParseFailed)
    }
}

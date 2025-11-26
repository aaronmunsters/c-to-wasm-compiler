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

pub trait FileOps {
    /// Create temporary file
    /// # Errors
    /// When temporary file creation fails
    fn create_temp(suffix: &str) -> std::io::Result<tempfile::NamedTempFile>;

    /// Writes content to path
    /// # Errors
    /// When writing fails
    fn write_all(file: &mut NamedTempFile, data: &[u8]) -> std::io::Result<()>;

    /// Reads content from path
    /// # Errors
    /// When reading fails
    fn read_file(path: &std::path::Path) -> std::io::Result<Vec<u8>>;
}

pub enum TempFS {}

impl FileOps for TempFS {
    fn create_temp(suffix: &str) -> std::io::Result<tempfile::NamedTempFile> {
        NamedTempFile::with_suffix(suffix)
    }

    fn write_all(file: &mut NamedTempFile, data: &[u8]) -> std::io::Result<()> {
        file.as_file().write_all(data)
    }

    fn read_file(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
        let output_file = File::open(path)?;
        let mut reader = BufReader::new(output_file);
        let mut output_content = vec![];
        reader.read_to_end(&mut output_content)?;
        Ok(output_content)
    }
}

pub type Compiler = AbstractCompiler<TempFS>;

pub struct AbstractCompiler<FS: FileOps> {
    _fs: core::marker::PhantomData<FS>,
}

impl<FS: FileOps> AbstractCompiler<FS> {
    /// Compiles the current configuration into a WebAssembly module using
    /// the [emscripten compiler](https://emscripten.org/).
    ///
    /// # Errors
    /// - If using the host's file system fails.
    /// - If compilation fails
    pub fn compile(configuration: &Configuration) -> Result<Vec<u8>, Error> {
        let mut input_source = FS::create_temp("_c-to-wasm-source.c").map_err(Error::IO)?;
        let output_wasm = FS::create_temp("_c-to-wasm-out.wasm").map_err(Error::IO)?;

        // Write into temp file
        FS::write_all(&mut input_source, configuration.source().as_bytes()).map_err(Error::IO)?;

        let mut command = configuration.as_command(input_source.path(), output_wasm.path());
        let output = command.output().map_err(Error::IO)?;

        if !output.status.success() {
            return Err(Error::Unsuccesful(output));
        }

        // Read from temp file
        let output_content = FS::read_file(output_wasm.path()).map_err(Error::IO)?;

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

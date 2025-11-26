use std::{path::Path, process::Command};

pub type Source = String;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    O0,
    O1,
    O2,
    O3,
}

impl Profile {
    fn as_emcc_arg(&self) -> &str {
        match self {
            Profile::O0 => "-O0",
            Profile::O1 => "-O1",
            Profile::O2 => "-O2",
            Profile::O3 => "-O3",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugging {
    Enabled,
    Disabled,
}

impl Debugging {
    fn as_emcc_arg(&self) -> &str {
        match self {
            Debugging::Enabled => "-g",
            Debugging::Disabled => "",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Configuration {
    pub(crate) profile: Profile,
    pub(crate) debugging: Debugging,
    pub(crate) source: String,
    pub(crate) filename: Filename,
}

#[derive(Clone, Debug)]
pub enum Filename {
    Unspecified,
    Configured(String),
}

impl Configuration {
    #[must_use]
    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    #[must_use]
    pub fn debugging(&self) -> &Debugging {
        &self.debugging
    }

    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

impl Configuration {
    pub(crate) fn as_command(&self, input_path: &Path, output_path: &Path) -> Command {
        let mut command = Command::new("emcc");
        // Set output path
        command.arg(input_path);
        // Disable requirement for `main` to be present
        command.arg("--no-entry");
        // Include performance profile
        command.arg(self.profile.as_emcc_arg());
        // Include debug flag if set in configuration
        command.arg(self.debugging.as_emcc_arg());
        // Disable generation of JS glue code
        command.args(["-s", "STANDALONE_WASM=1"]);
        // Set output path
        command.arg("-o").arg(output_path);

        command
    }
}

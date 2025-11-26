use c_to_wasm_compiler::{
    AbstractCompiler, FileOps,
    configuration::{Debugging, Profile},
    configuration_builder::ConfigurationBuilder,
};

static mut FAIL_COUNTER: i32 = 10;

const FAC_SOURCE: &str = include_str!("fac.c");

struct MockFS;

impl FileOps for MockFS {
    fn create_temp(suffix: &str) -> std::io::Result<tempfile::NamedTempFile> {
        use tempfile::NamedTempFile;
        if unsafe { FAIL_COUNTER == 0 } {
            Err(std::io::Error::from_raw_os_error(0))
        } else {
            unsafe { FAIL_COUNTER -= 1 }
            NamedTempFile::with_suffix(suffix)
        }
    }

    fn write_all(file: &mut tempfile::NamedTempFile, data: &[u8]) -> std::io::Result<()> {
        use std::io::Write;
        if unsafe { FAIL_COUNTER == 0 } {
            Err(std::io::Error::from_raw_os_error(0))
        } else {
            unsafe { FAIL_COUNTER -= 1 }
            file.as_file().write_all(data)
        }
    }

    fn read_file(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
        use std::fs::File;
        use std::io::{BufReader, Read};
        if unsafe { FAIL_COUNTER == 0 } {
            Err(std::io::Error::from_raw_os_error(0))
        } else {
            unsafe { FAIL_COUNTER -= 1 }
            let output_file = File::open(path)?;
            let mut reader = BufReader::new(output_file);
            let mut output_content = vec![];
            reader.read_to_end(&mut output_content)?;
            Ok(output_content)
        }
    }
}

#[test]
fn test_create_temp_file_error() {
    let run_with_fs_budget = |budget| {
        unsafe { FAIL_COUNTER = budget };

        let config = ConfigurationBuilder::init()
            .debugging(Debugging::Disabled)
            .profile(Profile::O0)
            .source(FAC_SOURCE.into())
            .build();

        AbstractCompiler::<MockFS>::compile(&config)
    };

    let max_fail_budget = 3;

    for failing_budget in 0..=max_fail_budget {
        run_with_fs_budget(failing_budget).unwrap_err();
    }

    run_with_fs_budget(max_fail_budget + 1).unwrap();
}

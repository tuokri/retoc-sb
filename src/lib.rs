use anyhow;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufWriter, Cursor};
use std::path::{Path, PathBuf};
use std::vec::Vec;

pub struct RetocContext {
    mounted_paths: Vec<PathBuf>,
}

pub struct RetocError;

impl RetocContext {
    pub fn mounted<P: AsRef<Path>>(mount_dirs: &[P]) -> anyhow::Result<RetocContext> {
        // TODO:
        let mut mounted_paths = vec![];
        for dir in mount_dirs {
            let mount_entries = fs::read_dir(dir)?
                .filter_map(anyhow::Result::ok)
                .map(|e| e.path())
                .filter(|p| p.extension() == Some(OsStr::new("utoc")));

            mounted_paths.extend(mount_entries);
        }

        Ok(RetocContext { mounted_paths })
    }
}

// TODO: rough usage:
// 1. create context
// 2. mount context to mount_dir (only once)
// 3. open specific input asset(s)
// 4. process specific input assets(s)
// 4.1. close input asset(s) once done
// 5. re-use context for next asset(s) [repeated]
// 6. cleanup & close

pub fn to_legacy(context: &RetocContext) -> anyhow::Result<()> {
    let mut cursor = Cursor::new(Vec::new());
    let mut _buf = BufWriter::new(cursor);

    Ok(())
}

pub fn to_zen(context: &RetocContext) {}

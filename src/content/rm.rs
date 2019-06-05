use std::fs;
use std::path::Path;

use ssri::Integrity;

use crate::content::path;
use crate::errors::Error;

pub fn rm(cache: &Path, sri: &Integrity) -> Result<(), Error> {
    fs::remove_file(path::content_path(&cache, &sri))?;
    Ok(())
}

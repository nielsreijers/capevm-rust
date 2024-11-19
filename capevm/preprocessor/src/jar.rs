use std::fs::File;
use std::io::{BufReader, Error, Read};
use zip::ZipArchive;

#[derive(Debug)]
pub(crate) struct JarReader {
    archive: ZipArchive<BufReader<File>>,
}

impl JarReader {
    pub(crate) fn new(path: &std::path::PathBuf) -> Result<JarReader, Error> {
        let f = File::open(path)?;
        let zip = ZipArchive::new(BufReader::new(f))?;
        Ok(JarReader { archive: zip })
    }

    pub fn classfile_names(&self) -> Vec<String> {
        self.archive
            .file_names()
            .filter(|name| name.ends_with(".class"))
            .map(|s| s.to_string())
            .collect()
    }

    pub fn by_filename(&mut self, filename: &str) -> Result<Vec<u8>, Error> {
        let zipfile = self.archive.by_name(filename)?;
        let contents = zipfile.bytes().map(|b| b.unwrap()).collect::<Vec<u8>>();
        Ok(contents)
    }
}

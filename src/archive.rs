use std::fs::{metadata, Metadata};
use std::io::{Error, Read, Write};
use std::path::Path;

#[derive(Debug)]
pub struct ArchiveFormat {
    pub(crate) filename: String,
    pub(crate) file_size: u64,
    pub(crate) contents: Option<Vec<u8>>,
}

impl ArchiveFormat {
    pub fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let filename = self.filename.as_bytes();
        let filename_len = filename.len() as u32;

        writer.write_all(&filename_len.to_le_bytes())?;
        writer.write_all(&filename)?;

        writer.write_all(&self.file_size.to_le_bytes())?;


        match &self.contents {
            Some(contents) => {
                writer.write_all(&contents.len().to_le_bytes())?;
                writer.write_all(&contents)?;
            }
            None => {}
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<ArchiveFormat> {
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;

        let filename_len = u32::from_le_bytes(len_bytes) as usize;

        let mut filename_bytes = vec![0u8; filename_len];
        reader.read_exact(&mut filename_bytes)?;

        let filename = String::from_utf8(filename_bytes);

        let mut file_size_bytes = [0u8; 8];
        reader.read_exact(&mut file_size_bytes)?;
        let file_size = u64::from_le_bytes(file_size_bytes);

        let mut content_size_bytes = [0u8; 8];
        reader.read_exact(&mut content_size_bytes)?;
        let content_size = u64::from_le_bytes(content_size_bytes);

        let mut contents = vec![0u8; content_size as usize];
        reader.read_exact(&mut contents)?;
        println!("{:?} | {:?}", filename, file_size);

        Ok(ArchiveFormat {
            filename: filename.unwrap(),
            file_size: file_size,
            contents: contents.into(),
        })
    }
}


pub fn build_archive_entry(file: &Path) -> Result<ArchiveFormat, Error> {
    let file_metadata: Metadata = metadata(file.to_str().unwrap())?;
    let filename = file.to_str().unwrap();

    Ok(ArchiveFormat {
        filename: filename.to_string(),
        file_size: file_metadata.len(),
        contents: None,
    })
}
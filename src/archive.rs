use std::fmt::format;
use std::fs::{metadata, Metadata};
use std::io::{Error, Read, Write};
use std::path::Path;
use jiff::Timestamp;

#[derive(Debug)]
pub struct ArchiveFormat {
    pub(crate) file_name: String,
    pub(crate) file_path: String,
    pub(crate) file_size: u64,
    pub(crate) contents: Option<Vec<u8>>,
}

impl ArchiveFormat {
    pub fn serialize<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let file_name = self.file_name.as_bytes();
        let file_name_len = file_name.len() as u32;

        writer.write_all(&file_name_len.to_le_bytes())?;
        writer.write_all(&file_name)?;

        let file_path = self.file_path.as_bytes();
        let file_path_len = file_path.len() as u32;

        writer.write_all(&file_path_len.to_le_bytes())?;
        writer.write_all(&file_path)?;

        writer.write_all(&self.file_size.to_le_bytes())?;
        
        match &self.contents {
            Some(contents) => {
                writer.write_all(&contents.len().to_le_bytes())?;
                writer.write_all(&contents)?;
            }
            None => {
                writer.write_all(&0u64.to_le_bytes())?;
            }
        }
        Ok(())
    }

    pub fn deserialize<R: Read>(reader: &mut R) -> std::io::Result<ArchiveFormat> {
        let mut len_bytes = [0u8; 4];
        reader.read_exact(&mut len_bytes)?;

        // FileName
        let file_name_len = u32::from_le_bytes(len_bytes) as usize;

        let mut file_name_bytes = vec![0u8; file_name_len];
        reader.read_exact(&mut file_name_bytes)?;

        let file_name = String::from_utf8(file_name_bytes);

        // FilePath
        reader.read_exact(&mut len_bytes)?;
        let file_path_len = u32::from_le_bytes(len_bytes) as usize;

        let mut file_path_bytes = vec![0u8; file_path_len];
        reader.read_exact(&mut file_path_bytes)?;

        let file_path = String::from_utf8(file_path_bytes);

        // FileSize
        let mut file_size_bytes = [0u8; 8];
        reader.read_exact(&mut file_size_bytes)?;
        let file_size = u64::from_le_bytes(file_size_bytes);

        // FileContents
        let mut content_size_bytes = [0u8; 8];
        reader.read_exact(&mut content_size_bytes)?;
        let content_size = u64::from_le_bytes(content_size_bytes);

        let mut contents = vec![0u8; content_size as usize];
        reader.read_exact(&mut contents)?;
        println!("{:?} | {:?}", file_name, file_size);

        Ok(ArchiveFormat {
            file_name: file_name.unwrap(),
            file_path: file_path.unwrap(),
            file_size: file_size,
            contents: contents.into(),
        })
    }
}


pub fn build_archive_entry(file: &Path) -> Result<ArchiveFormat, Error> {
    let file_metadata: Metadata = metadata(file.to_str().unwrap())?;
    let file_name = file
        .to_str()
        .and_then(|s| s.split('/').next_back())
        .map(str::to_string)
        .unwrap_or_else(|| {
            format!("out{}", Timestamp::now().strftime("%Y%m%d-%H%M"))
        });

    let file_path = file
        .to_str()
        .unwrap()
        .rsplit_once('/')
        .map(|(path, _)| path)
        .unwrap_or("");

    Ok(ArchiveFormat {
        file_name: file_name.to_string(),
        file_path: file_path.to_string(),
        file_size: file_metadata.len(),
        contents: None,
    })
}
use std::fs::{create_dir, File, create_dir_all, Permissions};
use std::io::{BufReader, Error, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use jiff::Timestamp;
use crate::archive::ArchiveFormat;

pub fn decode_archive(paths: Vec<PathBuf>) -> Result<(), Error> {
    for path in paths {
        let output_folder_name = path.file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.replace(".zrs", ""))
            .unwrap_or_else(|| format!("out{:}", Timestamp::now()
                .strftime("%Y%m%d-%H%M")));


        create_dir(output_folder_name.clone())?;
        let input = File::open(path.as_path())?;

        let mut reader = BufReader::new(input);

        let mut archive_entries: Vec<ArchiveFormat> = Vec::new();

        let mut count_bytes = [0u8; 4];
        reader.read_exact(&mut count_bytes)?;
        let entry_count = u32::from_le_bytes(count_bytes);

        for _ in 0..entry_count {
            let entry = ArchiveFormat::deserialize(&mut reader)?;
            archive_entries.push(entry);
        }

        for file_data in archive_entries {
            let output_path = Path::new(&output_folder_name).join(&file_data.file_path);
            create_dir_all(output_path.clone())?;

            let mut output = File::create(output_folder_name.to_string() + "/" + &*file_data.file_path + "/" + &*file_data.file_name)?;
            output.write(file_data.contents.unwrap().as_slice())?;
            output.flush()?;
            output.set_permissions(Permissions::from_mode(file_data.permissions))?;
        }
    }

    Ok(())
}


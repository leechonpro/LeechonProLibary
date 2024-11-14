use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write,SeekFrom,Seek};

pub struct FileIO {
    file_path: String,
    seek_position: SeekFrom,
}

impl FileIO {

    pub fn new(file_path: &str) -> Self {
        FileIO {
            file_path: file_path.to_string(),
            seek_position: SeekFrom::Start(0),
        }
    }

    pub fn write_to_file(&self, content: &str) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.file_path)?;

        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn write_to_new_file( &self, content: &str ) -> io::Result<()>
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;

        file.write_all(content.as_bytes())?;
        Ok(())

    }

    pub fn read_from_file(&mut self) -> io::Result<String> {
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();

        file.seek(self.seek_position)?;

        file.read_to_string(&mut content)?;

        self.seek_position = std::io::SeekFrom::Start(file.stream_position()?);

        Ok(content)
    }

    pub fn read_all_from_file(&self) -> io::Result<String> {
        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    pub fn reset_read_index( &mut self )
    {
        self.seek_position = std::io::SeekFrom::Start(0);
    }  
}
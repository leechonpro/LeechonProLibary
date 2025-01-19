use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write,SeekFrom,Seek};
use std::path::Path;

pub struct FileIO {
    file_path: String,
    seek_position: SeekFrom,
    file: Option<File>,
    last_char: char,
}

impl FileIO {

    pub fn new(file_path: &str) -> Self {
        FileIO {
            file_path: file_path.to_string(),
            seek_position: SeekFrom::Start(0),
            file:None,
            last_char: '\0'
        }
    }
    
    pub fn is_file_open( &mut self ) -> bool
    {
        match self.file
        {
            Some(_) => true,
            None => false,
        }
    }

    pub fn close_file( &mut self )
    {
        match &self.file
        {
            Some(file) =>
            {
                drop( file );
                self.file = None;
            },
            None =>
            {
                // warning. already closed.
            }
        }
    }

    pub fn open_file( &mut self )
    {
        match self.file
        {
            Some(_) => 
            { 
                // Warning. already opened.
            },
            None=>
            {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&self.file_path);
                match file
                {
                    Ok(file)=> {self.file = Some(file);},
                    Err(_)=>
                    {
                        // warning open failed
                    },
                }

            }
        }
    }

    pub fn open_new_file( &mut self )
    {
        match self.file
        {
            Some(_) => 
            { 
                // Warning. already opened.
            },
            None=>
            {
                let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.file_path);
                match file
                {
                    Ok(file)=> {self.file = Some(file);},
                    Err(_)=>
                    {
                        // warning open failed
                    },
                }

            }
        }
    }

    

    pub fn write_to_file(&mut self, content: &str) -> io::Result<()> 
    {
        match self.file
        {
            Some( ref mut file ) =>
            {
                file.write_all(content.as_bytes());
            },
            None =>
            {
                let mut file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(&self.file_path)?;
        
                file.write_all(content.as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn write_to_new_file( &mut self, content: &str ) -> io::Result<()>
    {
        let mut is_opened = false;
        match self.file
        {
            Some( ref mut file ) =>
            {
                self.close_file();
                is_opened = true;
            },
            None =>
            {
            }
        }
        let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&self.file_path);

        match file
        {
            Ok( mut new_file ) =>
            {
                new_file.write_all(content.as_bytes());
                if true == is_opened
                {
                    self.file = Some( new_file );
                }
            },
            Err(_)=>
            {
                
            }
        }
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

    pub fn is_exist(&self) -> bool
    {
        let file_path = Path::new(&self.file_path);

        file_path.exists()
    }

    fn p_read_byte(&mut self, file: &mut File ) -> char
    {
        let mut buffer = [0; 1]; 
        let result = file.read(&mut buffer);
        match result
        {
            Ok(size) => 
            {
                if 0 == size
                {
                    self.last_char = '\0';
                    return '\0';
                }
            },
            Error => 
            {
                self.last_char = '\0';
                return '\0';
            }
        }
        self.last_char = buffer[0] as char;
        self.last_char
    }

    pub fn read_bytes(&mut self, size: usize) -> String
    {
        let mut file_result = File::open(&self.file_path);
        let mut content = String::new();
        match file_result
        {
            Ok(mut file)=>
            {

                file.seek(self.seek_position);
                for _ in 0..size
                {
                    let ch = self.p_read_byte( &mut file );
                    content.push( ch );
                }
                let position_result = file.stream_position();
                match position_result
                {
                    Ok(position)=>self.seek_position = std::io::SeekFrom::Start(position),
                    Error=>{},
                }
            },
            Error=> {},
        }
        content        
    }

    pub fn read_line(&mut self) -> String
    {
        self.read_till_delimeters("\n".to_string() )
    }
    pub fn read_till_delimeters( &mut self, delimiters: String ) -> String
    {
        let mut file_result = File::open(&self.file_path);
        let mut content = String::new();
        match file_result
        {
            Ok(mut file)=>
            {

                file.seek(self.seek_position);
                let mut detect = false;
                loop
                {
                    let ch = self.p_read_byte( &mut file );
                    for delimiter in delimiters.chars()
                    {
                        if delimiter == ch
                        {
                            detect = true;
                        }
                    }

                    if ( '\0' == ch ) || ( true == detect ) 
                    {
                        break;
                    }
                    content.push( ch );
                }
                let position_result = file.stream_position();
                match position_result
                {
                    Ok(position)=>self.seek_position = std::io::SeekFrom::Start(position),
                    Error=>{},
                }
            },
            Error=> {},
        }
        content
    }

    pub fn is_end_of_file( &mut self ) -> bool
    {
        let mut file_result = File::open(&self.file_path);
        match file_result
        {
            Ok(mut file)=> {
                file.seek(self.seek_position);
                '\0' == self.p_read_byte( &mut file )
            },
            Error=> true,
        }
        
    }

    pub fn get_last_char( &mut self ) -> char
    {
        self.last_char
    }
}
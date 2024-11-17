//use std::ptr;
use byteorder::{ByteOrder, BigEndian};
//use crate::lib_app_config::AppConfig;

#[derive(Clone)]
pub struct DataBuffer
{
    pub data_vec: Vec<u8>,
    pub index: usize
}

impl DataBuffer
{
    pub fn new() -> Self
    {
        DataBuffer{ data_vec: Vec::new(), index: 0 }
    }

    pub fn set_u8( &mut self,input: u8 )
    {
        self.data_vec.push( input );
    }

    pub fn get_u8( &mut self ) -> u8
    {
        let mut result: u8 = 0;
        let size = self.data_vec.len();

        if self.index < size
        {
            result = self.data_vec[self.index];
            self.index += 1;
        }
        
        result
    }
    
    pub fn set_i8( &mut self,input: i8 )
    {
        self.data_vec.push( input as u8 );
    }

    pub fn get_i8( &mut self ) -> i8
    {
        let mut result: i8 = 0;
        let size = self.data_vec.len();

        if self.index < size
        {
            result = self.data_vec[self.index] as i8;
            self.index += 1;
        }
        
        result
    }    

    pub fn set_u16( &mut self,input: u16 )
    {

        self.data_vec.push( ( ( input & 0xff00 ) >> 8 ) as u8 );
        self.data_vec.push( ( input & 0x00ff ) as u8 );
    }

    pub fn get_u16( &mut self ) -> u16
    {
        let mut result: u16 = 0;
        let size = self.data_vec.len();

        if ( self.index + 1 ) < size
        {
            result |= ( self.data_vec[self.index] as u16 ) << 8 ;
            result |= self.data_vec[self.index + 1] as u16;
            self.index += 2;
        }
        
        result
    }
    
    pub fn set_i16( &mut self,input: i16 )
    {

        self.data_vec.push( ( ( ( input as u16 ) & 0xff00 ) >> 8 ) as u8 );
        self.data_vec.push( ( input & 0x00ff ) as u8 );
    }

    pub fn get_i16( &mut self ) -> i16
    {
        let mut result: i16 = 0;
        let size = self.data_vec.len();

        if ( self.index + 1 ) < size
        {
            result |= ( self.data_vec[self.index] as i16 ) << 8 ;
            result |= self.data_vec[self.index + 1] as i16;
            self.index += 2;
        }
        
        result
    }

    pub fn set_i32( &mut self,input: i32 )
    {
        self.data_vec.push( ( ( ( input as u32 ) & 0xff000000 ) >> 24 ) as u8 );
        self.data_vec.push( ( ( input & 0x00ff0000 ) >> 16 ) as u8 );
        self.data_vec.push( ( ( input & 0x0000ff00 ) >> 8 ) as u8 );
        self.data_vec.push( ( input & 0x000000ff ) as u8 );
    }

    pub fn get_i32( &mut self ) -> i32
    {
        let mut result: i32 = 0;
        let size = self.data_vec.len();

        if ( self.index + 3 ) < size
        {
            result |= ( self.data_vec[self.index] as i32 ) << 24 ;
            result |= ( self.data_vec[self.index + 1] as i32 ) << 16;
            result |= ( self.data_vec[self.index + 2] as i32 ) << 8 ;
            result |= self.data_vec[self.index + 3] as i32;
            self.index += 4;
        }
        
        result
    }

    
    pub fn set_u32( &mut self,input: u32 )
    {
        self.data_vec.push( ( ( input & 0xff000000 ) >> 24 ) as u8 );
        self.data_vec.push( ( ( input & 0x00ff0000 ) >> 16 ) as u8 );
        self.data_vec.push( ( ( input & 0x0000ff00 ) >> 8 ) as u8 );
        self.data_vec.push( ( input & 0x000000ff ) as u8 );
    }

    pub fn get_u32( &mut self ) -> u32
    {
        let mut result: u32 = 0;
        let size = self.data_vec.len();

        if ( self.index + 3 ) < size
        {
            result |= ( self.data_vec[self.index] as u32 ) << 24 ;
            result |= ( self.data_vec[self.index + 1] as u32 ) << 16;
            result |= ( self.data_vec[self.index + 2] as u32 ) << 8 ;
            result |= self.data_vec[self.index + 3] as u32;
            self.index += 4;
        }
        
        result
    }
    
    pub fn set_u64( &mut self,input: u64 )
    {
        self.set_u32( ( ( input & 0xffffffff00000000 ) >> 32 ) as u32 );
        self.set_u32( ( input & 0x00000000ffffffff ) as u32 );
    }

    pub fn get_u64( &mut self ) -> u64
    {
        let mut result: u64 = 0;
        let size = self.data_vec.len();

        if ( self.index + 7 ) < size
        {
            result |= ( self.data_vec[self.index] as u64 ) << 56 ;
            result |= ( self.data_vec[self.index] as u64 ) << 48 ;
            result |= ( self.data_vec[self.index] as u64 ) << 40 ;
            result |= ( self.data_vec[self.index] as u64 ) << 32 ;
            result |= ( self.data_vec[self.index] as u64 ) << 24 ;
            result |= ( self.data_vec[self.index + 1] as u64 ) << 16;
            result |= ( self.data_vec[self.index + 2] as u64 ) << 8 ;
            result |= self.data_vec[self.index + 3] as u64;
            self.index += 8;
        }
        
        result
    }

    pub fn set_f32( &mut self, input: f32 )
    {
        let mut bytes =  [0 as u8; 4];
        BigEndian::write_f32( &mut bytes, input );

        for data in bytes.iter()
        {
            self.data_vec.push( *data );
        }
    }

    pub fn get_f32( &mut self ) -> f32
    {
        let mut bytes = [0 as u8; 4];
        for idx in 0..4
        {
            bytes[idx] = self.get_u8();
        }
        
        BigEndian::read_f32( &mut bytes )
    }

    pub fn set_f64( &mut self, input: f64 )
    {
        let mut bytes =  [0 as u8; 8];
        BigEndian::write_f64( &mut bytes, input );

        for data in bytes.iter()
        {
            self.data_vec.push( *data );
        }
    }

    pub fn get_f64( &mut self ) -> f64
    {
        let mut bytes = [0 as u8; 8];
        for idx in 0..8
        {
            bytes[idx] = self.get_u8();
        }
        
        BigEndian::read_f64( &mut bytes )
    }

    pub fn set_char( &mut self, input :char )
    {
        self.data_vec.push( input as u8 );
    }

    pub fn get_char( &mut self ) -> char
    {
        let mut result: char = '\0';
        let size = self.data_vec.len();

        if self.index < size
        {
            result = self.data_vec[self.index] as char;
            self.index += 1;
        }
        
        result
    }

    pub fn set_string( &mut self, input: String )
    {
        let input_chars:Vec<char> = input.chars().collect();
        let input_len = input_chars.len();


        for index in 0..input_len
        {
            self.set_char( input_chars[index] );
        }
        self.set_char( '\0' );
    }

    pub fn get_string( &mut self ) -> String
    {
        let mut result = String::new();

        loop
        {
            let character = self.get_char();
            if '\0' == character
            {
                break;
            }
            result.push( character );
        }
        
        result
    }

    pub fn set_str( &mut self, input : &str )
    {
        self.set_string( String::from( input ) );
    }

    pub fn clear_buffer( &mut self )
    {
        self.data_vec.clear();
        self.index = 0;
    }

    pub fn set_buffer( &mut self, input: &mut DataBuffer )
    {
        let backup = input.index;

        input.reset_index();

        while !input.is_no_data()
        {
            self.set_u8( input.get_u8() );
        }
        input.index = backup;
    }

    pub fn reset_index( &mut self )
    {
        self.index = 0;
    }

    pub fn get_index( &mut self ) -> usize
    {
        self.index
    }

    pub fn is_empty( &mut self ) -> bool
    {
        self.data_vec.is_empty()
    }

    pub fn is_no_data( &mut self ) -> bool
    {
        self.data_vec.len() <= self.index
    }

    pub fn get_size( &mut self ) -> usize
    {
        self.data_vec.len()
    }

    pub fn get_buffer_vec( self ) -> Vec<u8>
    {
        self.data_vec
    }

    pub fn set_buffer_vec( &mut self, data : Vec<u8> )
    {
        self.data_vec = data;
    }
}

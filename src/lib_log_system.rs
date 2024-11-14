use crate::{Worker,DataBuffer,logging_level,DateTime};


pub trait Logging
{
    fn log_debug( &self, data : String );
    fn log_info( &self, data : String );
    fn log_warn( &self, data : String );
    fn log_error( &self, data : String );
}

pub struct LogSystem
{
    logging : Box<dyn Logging>,
    pub level : u16,
}

struct ConsoleLog
{
}

impl Logging for ConsoleLog
{
    fn log_debug( &self, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|DEBUG|{}",time.year, time.month, time.day, time.hour, time.minute,time.second,data );        
    }
    fn log_info( &self, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|INFO|{}",time.year, time.month, time.day, time.hour, time.minute,time.second,data );
    }
    fn log_warn( &self, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|WARN|{}",time.year, time.month, time.day, time.hour, time.minute,time.second,data );
    }
    fn log_error( &self, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|ERROR|{}",time.year, time.month, time.day, time.hour, time.minute,time.second,data );
    }

}

impl ConsoleLog
{
    fn new() -> Self
    {
        ConsoleLog{}
    }
}
impl LogSystem
{
    pub fn new( logging: Box<dyn Logging> ) -> Self
    {
        LogSystem{ logging, level: logging_level::DEBUG }
    }
    pub fn console() -> Self
    {
        let logging = Box::new( ConsoleLog::new() );
        LogSystem{ logging, level: logging_level::DEBUG }
    }
    pub fn log_debug( &mut self, text : String )
    {
        self.logging.log_debug( text );
    }
    pub fn log_info( &mut self, text : String )
    {
        self.logging.log_info( text );
    }
    pub fn log_warn( &mut self, text : String )
    {
        self.logging.log_warn( text );
    }
    pub fn log_error( &mut self, text : String )
    {
        self.logging.log_error( text );
    }
}

impl Worker for LogSystem
{
    fn update( &mut self ) -> bool
    {
        true
    }
    fn recv_event( &mut self, mut buffer: DataBuffer )
    {
        let event = buffer.get_u16();
        let text = buffer.get_string();
        if self.level >= event
        {
            match event
            {
                logging_level::INFO => { self.log_info( text ); },
                logging_level::DEBUG => { self.log_debug( text ); },
                logging_level::WARN => { self.log_warn( text ); },
                logging_level::ERROR => { self.log_error( text ); },
                _ => {},
            }
        }
    }
    fn initialize( &mut self )
    {
        
    }
    fn shutdown( &mut self )
    {

    }
}


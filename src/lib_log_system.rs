use crate::{Worker,DataBuffer,logging_level,DateTime,AppConfig};


pub trait Logging
{
    fn log_debug( &self, thread_id : String, data : String );
    fn log_info( &self, thread_id : String, data : String );
    fn log_warn( &self, thread_id : String, data : String );
    fn log_error( &self, thread_id : String, data : String );
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
    fn log_debug( &self, thread_id : String, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|DEBUG|{}|{}",time.year, time.month, time.day, time.hour, time.minute,time.second, thread_id, data );        
    }
    fn log_info( &self, thread_id : String, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|INFO|{}|{}",time.year, time.month, time.day, time.hour, time.minute,time.second, thread_id, data );
    }
    fn log_warn( &self, thread_id : String, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|WARN|{}|{}",time.year, time.month, time.day, time.hour, time.minute,time.second, thread_id, data );
    }
    fn log_error( &self, thread_id : String, data : String )
    {
        let time = DateTime::now();
        println!( "{}:{}:{}:{}:{}:{}|ERROR|{}|{}",time.year, time.month, time.day, time.hour, time.minute,time.second, thread_id, data );
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
    pub fn log_debug( &mut self, thread_id : String, text : String )
    {
        self.logging.log_debug( thread_id, text );
    }
    pub fn log_info( &mut self, thread_id : String, text : String )
    {
        self.logging.log_info( thread_id, text );
    }
    pub fn log_warn( &mut self, thread_id : String, text : String )
    {
        self.logging.log_warn( thread_id, text );
    }
    pub fn log_error( &mut self, thread_id : String, text : String )
    {
        self.logging.log_error( thread_id, text );
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
        let id = buffer.get_string();
        let text = buffer.get_string();
        if self.level >= event
        {
            match event
            {
                logging_level::INFO => { self.log_info( id, text ); },
                logging_level::DEBUG => { self.log_debug( id, text ); },
                logging_level::WARN => { self.log_warn( id, text ); },
                logging_level::ERROR => { self.log_error( id, text ); },
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


use std::thread;
use crate::DataBuffer;

pub trait Worker
{
    fn update( &mut self ) -> bool;
    fn recv_event( &mut self, buffer: DataBuffer );
    fn initialize( &mut self ); // Be called in ThreadWorker::new
    fn shutdown( &mut self ); //Be called in AooConfig::p_update_thread
}
pub struct ThreadWorker
{    
    pub worker : Box<dyn Worker>,
    pub handle : Option<thread::JoinHandle<()>>,
    pub is_running : bool,
}

impl ThreadWorker
{
    pub fn new( worker: Box<dyn Worker>, handle: Option<thread::JoinHandle<()>> ) -> Self
    {
        ThreadWorker{ worker, handle, is_running: true }
    }
}
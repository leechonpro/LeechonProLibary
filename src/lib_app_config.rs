use std::thread;
use std::time::Instant;
use std::sync::{mpsc, Arc, Mutex,Once};
use std::collections::HashMap;

use crate::{Worker,DataBuffer,ThreadWorker,LogSystem,module_id,logging_level,util};
//use crate::DataBuffer;

pub struct AppConfig
{
    app_name: String,
    version: String,
    message_queue: HashMap<module_id::ID,mpsc::Sender<( module_id::ID, DataBuffer )>>,
    thread_map: HashMap<module_id::ID, ThreadWorker>,
    tick_counter: Instant,
    read_queue:mpsc::Receiver<( module_id::ID, DataBuffer )>, 
}

//public function
impl AppConfig
{
// Common
    pub fn start()
    {        
        let _ = thread::spawn(||
        {
            loop
            {
                AppConfig::update();
                //thread::sleep(Duration::from_millis(100));
                std::thread::yield_now();
            }
        });
    }

    fn update( )
    {
        let binding = AppConfig::get_instance();
        let mut inst = binding.lock().unwrap();
        let( id, data ) = inst.p_pop_message();
        //let (id,mut data) = AppConfig::get_instance().lock().unwrap().p_pop_message();
        if id > 0
        {
            inst.p_push_message( id, data );
        }
        else
        {
            std::thread::yield_now();
        }
    }
    
    pub fn new() -> Self {
        let (send,read) = mpsc::channel();
        let mut message_queue = HashMap::new();
        message_queue.insert( 1, send.clone() );
        AppConfig {
            app_name: String::from("new project"),
            version: String::from("alpha"),
            message_queue: message_queue,
            thread_map: HashMap::new(),
            tick_counter:Instant::now(),
            read_queue:read,
        }
    }

    pub fn get_instance() -> Arc<Mutex<AppConfig>>
    {
        static ONCE: Once = Once::new();
        static mut INSTANCE: Option<Arc<Mutex<AppConfig>>> = None;
        
        // WARNING : unsafe code
        unsafe {
            ONCE.call_once(|| {
                INSTANCE = Some(Arc::new(Mutex::new(AppConfig::new())));
            });

            INSTANCE.as_ref().unwrap().clone()
        }
    }
// Program Info
    pub fn get_version() -> String
    {
        AppConfig::get_instance().lock().unwrap().p_get_version()
    }

    pub fn set_version( version: String )
    {
        AppConfig::get_instance().lock().unwrap().p_set_version( version );
    }

    pub fn get_app_name() -> String
    {
        AppConfig::get_instance().lock().unwrap().p_get_app_name()
    }

    pub fn set_app_name( app_name: String )
    {
        AppConfig::get_instance().lock().unwrap().p_set_app_name( app_name );
    }

// Multi threading
    pub fn push_message( id: module_id::ID, data: DataBuffer )
    {
        AppConfig::get_instance().lock().unwrap().p_push_message( id, data );
    }

    pub fn add_thread( id: module_id::ID, module : Box<dyn Worker> )
    {
        {
            AppConfig::get_instance().lock().unwrap().p_add_thread( id, module );
        }
        let mut is_not_ready = true;
        while is_not_ready
        {
            is_not_ready = !AppConfig::get_instance().lock().unwrap().p_is_pipe_available( id );    
        }
    }

    pub fn get_thread( id: module_id::ID ) -> Option<ThreadWorker>
    {
        AppConfig::get_instance().lock().unwrap().p_get_thread( id )
    }

    pub fn stop_thread( id: module_id::ID )
    {
        AppConfig::get_instance().lock().unwrap().p_stop_thread( id )
    }

// util
    pub fn get_tick_count() -> u64
    {
        AppConfig::get_instance().lock().unwrap().p_get_tick_count()
    }

// Pipe ( communication )
    pub fn create_pipe( id:module_id::ID ) -> mpsc::Receiver<( module_id::ID, DataBuffer )>
    {
        AppConfig::get_instance().lock().unwrap().p_create_pipe( id )
    }

    pub fn get_pipe( id:module_id::ID ) -> Option<mpsc::Sender<( module_id::ID, DataBuffer )>>
    {
        AppConfig::get_instance().lock().unwrap().p_get_pipe(id)
    }

    pub fn remove_pipe( id:module_id::ID )
    {
        AppConfig::get_instance().lock().unwrap().p_remove_pipe(id);
    }
// Logging
    pub fn default_logging()
    {
        {
            let log = LogSystem::console();
            AppConfig::get_instance().lock().unwrap().p_add_thread( module_id::LOGGING, Box::new( log ) );
        }
        let mut is_not_ready = true;
        while is_not_ready
        {
            is_not_ready = !AppConfig::get_instance().lock().unwrap().p_is_pipe_available( module_id::LOGGING );
        }
    }

    pub fn log_info( text: String )
    {
        let mut buffer = DataBuffer::new();
        buffer.set_u16( logging_level::INFO );
        buffer.set_string( util::get_thread_id() );
        buffer.set_string( text );

        AppConfig::get_instance().lock().unwrap().p_push_message( module_id::LOGGING, buffer );
    }
    
    pub fn log_debug( text: String )
    {
        let mut buffer = DataBuffer::new();
        buffer.set_u16( logging_level::DEBUG );
        buffer.set_string( util::get_thread_id() );
        buffer.set_string( text );

        AppConfig::get_instance().lock().unwrap().p_push_message( module_id::LOGGING, buffer );
        
    }
    
    pub fn log_warn( text: String )
    {
        let mut buffer = DataBuffer::new();
        buffer.set_u16( logging_level::WARN );
        buffer.set_string( util::get_thread_id() );
        buffer.set_string( text );

        AppConfig::get_instance().lock().unwrap().p_push_message( module_id::LOGGING, buffer );
        
    }
    
    pub fn log_error( text: String )
    {
        let mut buffer = DataBuffer::new();
        buffer.set_u16( logging_level::ERROR );
        buffer.set_string( util::get_thread_id() );
        buffer.set_string( text.clone() );

        AppConfig::get_instance().lock().unwrap().p_push_message( module_id::LOGGING, buffer );
    }

}

impl AppConfig
{
    fn p_get_version( &self ) -> String
    {
        self.version.clone()
    }

    fn p_set_version( &mut self, version: String )
    {
        self.version = version;
    }

    fn p_get_app_name( &self ) -> String
    {
        self.app_name.clone()
    }

    fn p_set_app_name( &mut self, app_name: String )
    {
        self.app_name= app_name;
    }

    fn p_push_message( &mut self, id: module_id::ID, data: DataBuffer )
    {        
        match self.message_queue.get( &id )
        {
            Some(test) => 
            { 
                let _ = test.send( ( id, data ) );
            },
            None => {},
        }
    }
    
    fn p_pop_message( &mut self ) -> ( module_id::ID, DataBuffer )
    {
        
        match self.read_queue.try_recv() {
            Ok(value) => value,
            Err(mpsc::TryRecvError::Empty) => ( 0,DataBuffer::new() ),
            Err(mpsc::TryRecvError::Disconnected) =>  ( 0,DataBuffer::new() ),
        }
    }

    fn p_get_thread( &mut self, id: module_id::ID ) -> Option<ThreadWorker>
    {
        self.thread_map.remove( &id ) // remove ThreadWorker's ownership
    }
    
    fn p_add_thread( &mut self, id: module_id::ID, mut module : Box<dyn Worker> )
    {
        let handle: Option<std::thread::JoinHandle<_>> = Some(thread::spawn(move||
            {
                let mut is_running = true;
                let mut is_need_init = true;
                let mut thread = AppConfig::get_thread( id ); //take thread's ownership
                while is_need_init
                {
                    match thread
                    {
                        Some( ref mut test ) => 
                        {
                            let read = AppConfig::create_pipe( id );
                            while is_running
                            {
                                
                                match read.try_recv() {
                                    Ok((id,buffer)) => {
                                        if 0 == id
                                        {
                                            test.is_running = false;
                                        }
                                        else
                                        {
                                            test.worker.recv_event(buffer); 
                                        }
                                    },
                                    Err(mpsc::TryRecvError::Empty) => {
                                    //    println!("is empty");
                                    },
                                    Err(mpsc::TryRecvError::Disconnected) => {},
                                }
                                is_running = test.worker.update();
                                if is_running
                                {
                                    is_running = test.is_running;
                                }

                                if false == is_running
                                {
                                    test.worker.shutdown();
                                }
                                std::thread::yield_now();

                            }
                            is_need_init = false;
                        }
                        None => { thread = AppConfig::get_thread( id ); }, //take thread's ownership
                    }

                }
            }));
            
        module.initialize();
        self.thread_map.insert( id, ThreadWorker::new( module, handle ) );


    }

    fn p_stop_thread( &mut self, id:module_id::ID )
    {
        match self.message_queue.get( &id )
        {
            Some(test) => 
            { 
                let _ = test.send( ( 0, DataBuffer::new() ) );
            },
            None => {},
        }
    }

    fn p_get_tick_count( &mut self ) -> u64
    {
        let elapsed = self.tick_counter.elapsed();
        elapsed.as_millis() as u64
    }

    fn p_register_pipe( &mut self, id:module_id::ID, channel:mpsc::Sender<( module_id::ID, DataBuffer )>)
    {
        self.message_queue.insert(id, channel );
    }

    fn p_unregister_pipe( &mut self, id:module_id::ID )
    {
        self.message_queue.remove(&id);
    }

    fn p_create_pipe( &mut self, id:module_id::ID ) -> mpsc::Receiver<( module_id::ID, DataBuffer )>
    {
        let (send,read) = mpsc::channel();
        self.p_register_pipe( id, send );
        read
    }

    fn p_get_pipe( &mut self, id:module_id::ID ) -> Option<mpsc::Sender<( module_id::ID, DataBuffer )>>
    {
        self.message_queue.get(&id).cloned()
    }

    fn p_is_pipe_available( &mut self, id:module_id::ID ) -> bool
    {
        match self.message_queue.get(&id)
        {
            Some(_) => true,
            None => false,
        }
    }
    
    fn p_remove_pipe( &mut self, id:module_id::ID )
    {
        self.message_queue.remove(&id);
    }

}

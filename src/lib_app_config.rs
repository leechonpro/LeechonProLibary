use std::thread;
use std::time::Instant;
use std::sync::{mpsc, Arc, Mutex,Once};
use std::collections::HashMap;

use crate::{Worker,DataBuffer,ThreadWorker};
//use crate::DataBuffer;
type ModuleID = i32;

pub struct AppConfig
{
    app_name: String,
    version: String,
    message_queue: HashMap<ModuleID,mpsc::Sender<( ModuleID, DataBuffer )>>,
    thread_map: HashMap<ModuleID, ThreadWorker>,
    tick_counter: Instant,
    read_queue:mpsc::Receiver<( ModuleID, DataBuffer )>,
}

//public function
impl AppConfig
{
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

    fn update()
    {
        let (id,mut data) = AppConfig::get_instance().lock().unwrap().p_pop_message();
        if id > 0
        {
            let text = data.get_string();
            println!("{}", text);
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
        
        // unsafe code
        unsafe {
            ONCE.call_once(|| {
                INSTANCE = Some(Arc::new(Mutex::new(AppConfig::new())));
            });

            INSTANCE.as_ref().unwrap().clone()
        }
    }

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

    pub fn push_message( id: ModuleID, data: DataBuffer )
    {
        AppConfig::get_instance().lock().unwrap().p_push_message( id, data );
    }

    pub fn add_thread( id: ModuleID, module : Box<dyn Worker> )
    {
        AppConfig::get_instance().lock().unwrap().p_add_thread( id, module );        
    }
    pub fn get_thread( id: ModuleID ) -> Option<ThreadWorker>
    {
        AppConfig::get_instance().lock().unwrap().p_get_thread( id )
    }
    pub fn stop_thread( id: ModuleID )
    {
        AppConfig::get_instance().lock().unwrap().p_stop_thread( id )
    }
    pub fn get_tick_count() -> u64
    {
        AppConfig::get_instance().lock().unwrap().p_get_tick_count()
    }
    pub fn create_channel( id:ModuleID ) -> mpsc::Receiver<( ModuleID, DataBuffer )>
    {
        AppConfig::get_instance().lock().unwrap().p_create_channel( id )
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

    fn p_push_message( &mut self, id: ModuleID, data: DataBuffer )
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
    
    fn p_pop_message( &mut self ) -> ( ModuleID, DataBuffer )
    {
        
        match self.read_queue.try_recv() {
            Ok(value) => value,
            Err(mpsc::TryRecvError::Empty) => ( 0,DataBuffer::new() ),
            Err(mpsc::TryRecvError::Disconnected) =>  ( 0,DataBuffer::new() ),
        }
    }

    fn p_get_thread( &mut self, id: ModuleID ) -> Option<ThreadWorker>
    {
        self.thread_map.remove( &id ) // remove ThreadWorker's ownership
    }
    
    fn p_add_thread( &mut self, id: ModuleID, mut module : Box<dyn Worker> )
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
                            let read = AppConfig::create_channel( id );
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
                                    Err(mpsc::TryRecvError::Empty) => {},
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

    fn p_stop_thread( &mut self, id:ModuleID )
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

    fn p_save_read_channel( &mut self, id:ModuleID, channel:mpsc::Sender<( ModuleID, DataBuffer )>)
    {
        self.message_queue.insert(id, channel );
    }

    fn p_create_channel( &mut self, id:ModuleID ) -> mpsc::Receiver<( ModuleID, DataBuffer )>
    {
        let (send,read) = mpsc::channel();
        self.p_save_read_channel( id, send );
        read
    }
}

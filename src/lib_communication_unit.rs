use std::net::{TcpStream,TcpListener,Shutdown};
use std::io::{Read,Write, ErrorKind};
use std::collections::{HashMap,VecDeque};

use crate::{AppConfig, DataBuffer, sock_val};

pub trait CommunicationUnit
{
    fn connect( &mut self ) -> bool;
    fn disconnect( &mut self );
    fn write( &mut self, id: u32, buffer: &mut DataBuffer );
    fn read( &mut self ) -> ( u32, DataBuffer );
}

pub struct ClientSocket
{
    socket :Option<TcpStream>,
    ip_address :String,
    port: u16,
    is_server: bool,
}

impl ClientSocket
{
    pub fn new( ip_address: String, port: u16 ) -> Self
    {
        ClientSocket
        { 
            socket: None,
            ip_address,port,
            is_server : false,
        }
    }

    fn set_nonblocking( &mut self, is_non_block: bool )
    {
        match &self.socket
        {
            Some( sock ) => 
            {
                sock.set_nonblocking( is_non_block ).expect( "set_nonblocking call failed" );
            },
            None => 
            {
                // socket is not set
            }
        }
    }

    fn set_socket( socket: Option<TcpStream>)->Self
    {
        match socket
        {
            Some(ref sock) => 
            {
                sock.set_nonblocking( true );
            },
            None => 
            {
                AppConfig::log_warn( String::from( "socket is empty" ) );
            },
        }
        ClientSocket
        { 
            socket, 
            ip_address: String::from( "" ),
            port: 0,
            is_server : true,
        }
    }
}

impl CommunicationUnit for ClientSocket
{
    fn connect( &mut self ) -> bool
    {
        self.disconnect();
        self.socket = Some( TcpStream::connect( &(format!("{}:{}", self.ip_address, self.port) )).expect("ClientSocket::connect") );
        AppConfig::log_debug(format!( "try to connect : {}",format!("{}:{}", self.ip_address, self.port) ) );
        match &self.socket
        {
            Some(_) => {
                self.set_nonblocking( true );
                return true;
            },
            None=> { 
                AppConfig::log_error(String::from( "client connect failed") );
                return false;
            },
        }
        
        false
    }

    fn disconnect( &mut self )
    {
        match &self.socket
        {
            Some(sock) => {
                //shutdown first.
                let _ = sock.shutdown(Shutdown::Both);
            },
            None=> {},
        }
        self.socket = None;
    }

    fn write( &mut self, id: u32, buffer: &mut DataBuffer )
    {
        match &mut self.socket
        {
            Some(soc) => {
                let mut packet = DataBuffer::new();
                packet.set_u16( sock_val::PACKET_CHECK ); //to check, 
                packet.set_u16( buffer.get_size() as u16 ); // set size;
                packet.set_buffer( buffer );
                soc.write(&packet.get_buffer_vec()).expect("ClientSocket::write");
            },
            None=> {},
        }
    }

    fn read( &mut self ) -> ( u32, DataBuffer )
    {
        match &mut self.socket
        {
            Some( sock ) => {
                let mut header = vec![0;sock_val::PACKET_HEAD_SIZE];
                match sock.read(&mut header) {
                    Ok(_) => 
                    {
                        //front info
                        let code = ( header[0] as u16 ) << 8 | ( header[1] as u16 );  
                        //size
                        let size = ( header[2] as usize ) << 8 | ( header[3] as usize );
                        
                        if sock_val::PACKET_CHECK == code
                        {
                            let mut buf = vec![0;size];
                            match sock.read(&mut buf) {
                                Ok(_) => 
                                {
                                    let mut buffer = DataBuffer::new();
                                    buffer.set_buffer_vec( buf.clone() );
                                    return ( 
                                        if self.is_server{sock_val::CLIENT} else {sock_val::SERVER},
                                        buffer );
                                },
                                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                                    //No data                                
                                },
                                Err(_e) => 
                                {
                                    //Disconnected
                                },
                            }
                        }

                    },
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        //No header                        
                    },
                    Err(_e) => 
                    {
                        //Disconnected
                    },
                }
            },
            None=> 
            {
                //No connection
            },
        }

        ( if self.is_server{sock_val::SERVER} else {sock_val::CLIENT}, DataBuffer::new() )
    }
}

pub struct ServerSocket
{
    socket :Option<TcpListener>,
    client_list: HashMap<u32,ClientSocket>,
    ip_address: String,
    port: u16,
    message_queue: VecDeque<(u32, DataBuffer)>,
    next_id:u32,
}

impl ServerSocket
{
    pub fn new( ip_address:String, port: u16 ) ->Self
    {
        ServerSocket{
            socket :None,
            client_list:HashMap::new(),
            ip_address,
            port,
            message_queue: VecDeque::new(),
            next_id: sock_val::CLIENT,
        }
    }

    fn bind( &mut self )
    {
        match &self.socket
        {
            Some(_) => 
            {
                // server already binded
            },
            None =>
            {
                match TcpListener::bind(&(format!("{}:{}", self.ip_address, self.port)))
                {
                    Ok(sock) => 
                    {
                        sock.set_nonblocking(true).expect("set_nonblocking call failed");
                        self.socket = Some(sock);
                        AppConfig::log_debug( format!( "server binding success: {}",format!("{}:{}", self.ip_address, self.port) ) );
                    },
                    Err(_e) => 
                    {
                        AppConfig::log_warn( format!( "server binding failed: {}",format!("{}:{}", self.ip_address, self.port) ) );
                    }
                }

            }
        }
    }
}

impl CommunicationUnit for ServerSocket
{
    fn connect( &mut self ) -> bool
    {
        let mut is_connected = false;
        self.bind();
        match &self.socket
        {
            Some(sock) => 
            {
                for stream in sock.incoming() {
                    match stream {
                        Ok(stream) => 
                        {
                            let client = ClientSocket::set_socket( Some(stream) );
                            self.client_list.insert(self.next_id, client );
                            AppConfig::log_info(format!( "client id : {} connected", self.next_id) );
                            self.next_id += 1;
                            is_connected = true;
                        },
                        Err(_e) => 
                        {
                            break;
                        }
                    }
                }
            },
            None =>
            {
                AppConfig::log_error(String::from( "need to bind"));
            }
        }
        
        is_connected
    }

    fn disconnect( &mut self )
    {
        for (_, socket) in &mut self.client_list
        {
            socket.disconnect();
        }
        self.client_list.clear();
    }

    fn write( &mut self, id: u32, buffer: &mut DataBuffer )
    {
        if sock_val::SERVER == id
        {
            //sent to all. 
        }
        else
        {
            match self.client_list.get_mut( &id )
            {
                Some(test) => 
                { 
                    let _ = test.write( id, buffer );
                },
                None => {
                    AppConfig::log_warn(format!("send failed id : {}", id ));
                },
            }
        }
    }

    fn read( &mut self ) -> ( u32, DataBuffer )
    {
        for (client_id, socket) in &mut self.client_list
        {
            let (id, message) = socket.read();
            if id == sock_val::CLIENT
            {
                self.message_queue.push_back( (*client_id, message) );
            }
        }

        if 0 < self.message_queue.len()
        {
            self.message_queue.pop_front().expect("ServerSocket::read")
        }
        else
        {
            ( sock_val::SERVER, DataBuffer::new() )
        }
    }
    
}
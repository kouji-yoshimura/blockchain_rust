use std::{
    net::{TcpListener, TcpStream},
    io::{Read, Write},
    collections::HashMap,
};

use base64::{engine::general_purpose, Engine as _};
use sha1::{Digest, Sha1};
use crate::websocket::{
    frame::Frame,
    opcode::Opcode,
};

pub mod opcode;
pub mod frame;
pub const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

enum ConnectionPhase {
    Handshake,
    Open,
}

pub struct WebSocketContext<'a> {
    pub stream: &'a mut TcpStream,
    pub frame: Frame,
}

pub struct WebSocketBuilder {
    endpoint: Option<String>,
    operations: HashMap<Opcode, fn(WebSocketContext)>,
}

impl WebSocketBuilder {
    pub fn endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }

    pub fn add_operation(mut self, opcode: Opcode, func: fn(WebSocketContext)) -> Self {
        self.operations.insert(opcode, func);
        self
    }

    pub fn build(&self) -> WebSocket {
        WebSocket::new(
            self.endpoint.clone().unwrap(),
            self.operations.clone(),
            ConnectionPhase::Handshake,
        )
    }
}

pub struct WebSocket {
    endpoint: String,
    operations: HashMap<Opcode, fn(WebSocketContext)>,
    connection_phase: ConnectionPhase,
}

impl WebSocket {
    fn new(
        endpoint: String,
        operations: HashMap<Opcode, fn(WebSocketContext)>,
        connection_phase: ConnectionPhase
    ) -> Self {
        WebSocket {
            endpoint,
            operations,
            connection_phase,
        }
    }

    pub fn builder() -> WebSocketBuilder {
        WebSocketBuilder {
            endpoint: None,
            operations: HashMap::new(),
        }
    }

    pub fn listen(&mut self) {
        let listener = TcpListener::bind(self.endpoint.clone()).unwrap();
        let mut buffer = [0; 4096];

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            loop {
                println!("listening...");
                stream.read(&mut buffer).unwrap();
                match self.connection_phase {
                    ConnectionPhase::Handshake => self.opening_handshake(&mut stream, buffer),
                    ConnectionPhase::Open => self.websocket_connection(&mut stream, buffer),
                };
            }
        }
    }

    fn opening_handshake(&mut self, stream: &mut TcpStream, buffer: [u8; 4096]) {
        let mut request_headers = HashMap::new();

        let request_text = String::from_utf8_lossy(&buffer[..]);
        for (i, line) in request_text.lines().enumerate() {
            if line == "" {
                break;
            }

            if i == 0 {
                let values = line.split(" ").map(|s| s.trim()).collect::<Vec<&str>>();
                request_headers.insert("method".to_string(), values[0].to_string());
                request_headers.insert("uri".to_string(), values[1].to_string());
                request_headers.insert("protocol_version".to_string(), values[2].to_string());
                continue;
            }

            let values = line.split(":").map(|s| s.trim()).collect::<Vec<&str>>();
            request_headers.insert(values[0].to_lowercase().to_string(), values[1].to_string());
        }

        let mut hasher = Sha1::new();
        hasher.update(format!("{}{}", request_headers.get("sec-websocket-key").unwrap(), WEBSOCKET_GUID));
        let sec_websocket_accept = general_purpose::STANDARD.encode(hasher.finalize());

        let response = format!("{}\r\n{}\r\n{}\r\n{}{}\r\n\r\n",
            "HTTP/1.1 101 OK",
            "Upgrade: websocket",
            "Connection: Upgrade",
            "Sec-WebSocket-Accept: ", sec_websocket_accept,
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

        self.connection_phase = ConnectionPhase::Open;
    }

    fn websocket_connection(&mut self, stream: &mut TcpStream, buffer: [u8; 4096]) {
        let frame = Frame::from(&buffer[..]);
        match frame.opcode() {
            Opcode::Text => {
                if let Some(func) = self.operations.get(&Opcode::Text) {
                    func(WebSocketContext { stream, frame });
                }
            },
            Opcode::Close => {
                let response = Frame::new(Opcode::Close, None);
                stream.write(&response.to_bytes()).unwrap();
                stream.flush().unwrap();

                println!("closed");
                self.connection_phase = ConnectionPhase::Handshake;
            }
            _ => {},
        };
    }
}

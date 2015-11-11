extern crate ws;

use ws::{listen, Sender, Handler, Message, Result};

fn main() {

    struct Server {
        out: Sender,
    }

    impl Handler for Server {

        fn on_message(&mut self, msg: Message) -> Result<()> {
            println!("Server got message '{}'. ", msg);
            self.out.broadcast(msg)
        }
    }

    listen("127.0.0.1:2794", |out| {
        Server { out: out }
    }).unwrap()

}

//extern crate websocket;
//extern crate crossbeam;

//use std::thread;
//use std::sync::mpsc::channel;
//use std::sync::Arc;
//use std::sync::Mutex;
//use websocket::{Server, Message, Sender, Receiver};
//use websocket::message::Type;
//use websocket::header::WebSocketProtocol;

//fn main() {
    //let server = Server::bind("127.0.0.1:2794").unwrap();
    //let local_server = server.try_clone().unwrap();

    //crossbeam::scope(|scope| {

    //for connection in server {

        //scope.spawn(move || {
            //let request = connection.unwrap().read_request().unwrap();
            //let headers = request.headers.clone();

            //request.validate().unwrap();

            //let mut response = request.accept();

            //if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                //if protocols.contains(&("rust-websocket".to_string())) {
                    //response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                //}
            //}

            //let mut client = response.send().unwrap();
            
            //let ip = client.get_mut_sender()
                           //.get_mut()
                           //.peer_addr()
                           //.unwrap();

            //println!("Connection from {}", ip);

            //let message: Message = Message::text("Hello".to_string());
            //client.send_message(&message).unwrap();

            //let (mut sender, mut receiver) = client.split();

            //for message in receiver.incoming_messages() {
                //let message: Message = message.unwrap();

                //match message.opcode {
                    //Type::Close => {
                        //let message = Message::close();
                        //sender.send_message(&message).unwrap();
                        //println!("Client {} disconnected", ip);
                        //return;
                    //},
                    //Type::Ping => {
                        //let message = Message::pong(message.payload);
                        //sender.send_message(&message).unwrap();
                    //}
                    //_ => {
                        //// broadcast message to all clients
                        //local_server.try_clone();
                        //sender.send_message(&message).unwrap()
                    //},
                //}
            //}
        //});
    //}
    //});
//}

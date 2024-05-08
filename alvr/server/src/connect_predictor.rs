// use std::net::TcpStream;
// use std::io::{Read, Write};




// pub fn get_motion_from_alvr(motion:Vec<(u64, alvr_common::DeviceMotion)>){
//         println!("get_motion_from_alvr calle");
//         // println!("get motion {:?}", motion);
//     }





// fn main() {
//     // Define host and port
//     let host = "127.0.0.1"; // Loopback address for localhost
//     let port = 12345;       // Same port as the server
    
//     get_motion_from_alvr(motion);
//     // Connect to the server
//     match TcpStream::connect((host, port)) {
//         Ok(mut stream) => {
//             println!("Connected to server at {}:{}", host, port);

//             // Send data to server
//             let message = "Hello, server!";
//             stream.write_all(message.as_bytes()).unwrap();
//             println!("Sent message to server: {}", message);

//             // Receive response from server
//             let mut buffer = [0; 1024];
//             match stream.read(&mut buffer) {
//                 Ok(_) => {
//                     let response = String::from_utf8_lossy(&buffer);
//                     println!("Received response from server: {}", response);
//                 },
//                 Err(err) => {
//                     eprintln!("Error receiving response: {:?}", err);
//                 }
//             }
//         },
//         Err(err) => {
//             eprintln!("Failed to connect to server: {:?}", err);
//         }
//     }
// }

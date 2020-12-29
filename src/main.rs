use std::net::UdpSocket;
use std::convert::TryInto;
use enigo::*;


//This function combines the 2 8 bit integers into one 16 bit int
fn combine_bytes(b1: u8, b2: u8) -> i32 {
    return ((b1 as i32) << 8 ) | ((b2 as i32) & 0xff);
}

fn main() -> std::io::Result<()> {
    #[allow(unused_must_use)] {
        let socket = UdpSocket::bind("0.0.0.0:40118")?;
        
        loop {
            //The data will always be at most, 20 bytes
            let mut data = [0; 20];

            // Receive UDP data and put it into the buffer
            socket.recv_from(&mut data);
            
            //The first 12 bytes of data aren't worth keeping, (contain versioning info) so they're discarded
            let data = &data[12..];
            
            //Now some useful stuff
            //First, combine all the 8 bit integers into 16 bit ones
            let mut x: i32 = combine_bytes(data[0], data[1]);
            let mut y: i32 = combine_bytes(data[2], data[3]);
            let pressure: u16 = combine_bytes(data[4], data[5]).try_into().unwrap();

            //Then, scale them to (my) screen dimensiosn
            x = (((x as f32)/65535.0) * 1366.0) as i32;
            y = (((y as f32)/65535.0) * 768.0) as i32;
            println!("x: {:?} y: {:?}", x, y);
            println!("pressure: {:?} ", pressure);


            let mut enigo = Enigo::new();

            if pressure > 0 {
                enigo.mouse_down(MouseButton::Left)
            } else {
                enigo.mouse_up(MouseButton::Left);
                enigo.key_down(Key::Control);
                enigo.key_click(Key::Layout('w'));
                enigo.key_up(Key::Control);

            };

            enigo.mouse_move_to(x, y);
        }
    } 
}


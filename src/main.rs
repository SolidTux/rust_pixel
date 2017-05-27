extern crate image;

use std::io::prelude::*;
use std::io::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let nthreads = 1000;

    loop {
        let x0 = 1250;
        let y0 = 400;
        let mut handles = Vec::new();
        println!("→ {} {}", x0, y0);
        for n in 0..nthreads {
            let xoff = n*1920/nthreads;
            handles.push(thread::spawn(move || {
                loop {
                    match TcpStream::connect("94.45.231.39:1234") {
                        Ok(tcp) => {
                            tcp.set_nodelay(true).expect("set_nodelay call failed");
                            let mut stream = BufWriter::new(tcp);
                            let mut y = 0;
                            let mut x = xoff;
                            loop {
                                y = (y+1) % 1200;
                                if y == 0 {
                                    x = (x+1) % 1920;
                                }
                                match stream.write_fmt(format_args!("PX {} {} 00FF00\n", x, y)) {
                                    Ok(_) => {},
                                    Err(_) => {
                                        println!("Write error, connecting again ..");
                                        break
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            println!("Connection error, trying again ...");
                            thread::sleep(Duration::from_secs(1))
                        }
                    }
                }
            }));
        }
        for h in handles.into_iter() {
            h.join().unwrap();
        }
    }
}

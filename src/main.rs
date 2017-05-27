extern crate image;

use image::*;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let nthreads = 300;
    let image = open("image.png").expect("image").to_rgba();

    loop {
        let x0 = 1700;
        let y0 = 700;
        let mut handles = Vec::new();
        println!("→ {} {}", x0, y0);
        for xoff in 0..nthreads {
            let img = image.clone();
            handles.push(thread::spawn(move || {
                loop {
                    match TcpStream::connect("94.45.231.39:1234") {
                        Ok(mut stream) => {
                            stream.set_nodelay(true).expect("set_nodelay call failed");
                            let mut y = 0;
                            let mut x = xoff;
                            loop {
                                y = (y+1) % img.height();
                                if y == 0 {
                                    x = (x+xoff) % img.width();
                                }
                                let (r, g, b, a) = img.get_pixel(x, y).channels4();
                                if a > 10 {
                                    match stream.write_fmt(format_args!("PX {} {} {:02X}{:02X}{:02X}\n", x0+x, y0+y, r, g, b)) {
                                        Ok(_) => {},
                                        Err(_) => {
                                            println!("Write error, connecting again ..");
                                            break
                                        }
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

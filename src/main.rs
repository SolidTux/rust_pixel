extern crate image;
extern crate rand;
//extern crate time;

use std::io::prelude::*;
use std::net::TcpStream;
use image::*;
use rand::distributions::{IndependentSample, Range};
//use time::PreciseTime;
use std::thread;

fn main() {
    let image = open("image.png").expect("image").to_rgba();
    //let x0gen = Range::new(0,1920-image.width());
    //let y0gen = Range::new(0,1200-image.height());
    let xoffgen = Range::new(0,image.width());
    let yoffgen = Range::new(0,image.height());
    let nthreads = 333;
    let mut rng = rand::thread_rng();

    loop {
        let x0 = 50;
        let y0 = 300;
        //let x0 = x0gen.ind_sample(&mut rng);
        //let y0 = y0gen.ind_sample(&mut rng);
        let mut handles = Vec::new();
        println!("→ {} {}", x0, y0);
        for _ in 0..nthreads {
            let xoff = xoffgen.ind_sample(&mut rng);
            let yoff = yoffgen.ind_sample(&mut rng);
            let img = image.clone();
            handles.push(thread::spawn(move || {
                let mut stream = TcpStream::connect("94.45.231.39:1234").expect("connection");
                stream.set_nodelay(true).expect("set_nodelay call failed");
                let mut x = xoff;
                let mut y = yoff;
                //let stime = PreciseTime::now();
                loop {
                    //let diff = stime.to(PreciseTime::now()).num_seconds();
                    //if diff > 20 {
                        //break;
                    //}
                    y = (y+1) % img.height();
                    if y == 0 {
                        x = (x+1) % img.width();
                    }
                    let (r, g, b, a) = img.get_pixel(x, y).channels4();
                    if a > 10 {
                        let _ = stream.write_fmt(format_args!("PX {} {} {:02X}{:02X}{:02X}\n", x0+x, y0+y, r, g, b));
                    }
                }
            }));
            let img = image.clone();
            handles.push(thread::spawn(move || {
                let mut stream = TcpStream::connect("94.45.231.39:1234").expect("connection");
                stream.set_nodelay(true).expect("set_nodelay call failed");
                let mut x = xoff;
                let mut y = yoff;
                //let stime = PreciseTime::now();
                loop {
                    //let diff = stime.to(PreciseTime::now()).num_seconds();
                    //if diff > 5 {
                        //break;
                    //}
                    x = (x+1) % img.width();
                    y = (y+1) % img.height();
                    let (r, g, b, a) = img.get_pixel(x, y).channels4();
                    if a > 10 {
                        let _ = stream.write_fmt(format_args!("PX {} {} {:02X}{:02X}{:02X}\n", x0+x, y0+y, r, g, b));
                    }
                }
            }));
            let img = image.clone();
            handles.push(thread::spawn(move || {
                let mut stream = TcpStream::connect("94.45.231.39:1234").expect("connection");
                stream.set_nodelay(true).expect("set_nodelay call failed");
                let mut x = xoff;
                let mut y = yoff;
                //let stime = PreciseTime::now();
                loop {
                    //let diff = stime.to(PreciseTime::now()).num_seconds();
                    //if diff > 5 {
                        //break;
                    //}
                    x = (x+1) % img.width();
                    if x == 0 {
                        y = (y+1) % img.height();
                    }
                    let (r, g, b, a) = img.get_pixel(x, y).channels4();
                    if a > 10 {
                        let _ = stream.write_fmt(format_args!("PX {} {} {:02X}{:02X}{:02X}\n", x0+x, y0+y, r, g, b));
                    }
                }
            }));
        }
        for h in handles.into_iter() {
            h.join().unwrap();
        }
    }
}

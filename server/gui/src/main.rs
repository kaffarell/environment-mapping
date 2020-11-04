use nalgebra::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use posixmq::PosixMq;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

type Point = (f32, f32, f32);

fn get_degrees(mq: &PosixMq) -> f32 {
    let mut buf = vec![0; mq.attributes().max_msg_len];
    println!("{:?}", mq.receive(&mut buf).unwrap());
    let mut mod_buf = String::new();
    for i in buf {
        if i == 0 {
            break;
        }else {
            mod_buf.push_str(&(i - 48).to_string());
        }
    }
    let degrees: i32 = mod_buf.parse::<i32>().unwrap();
    return degrees as f32;
}

fn main() {
    // Create kiss3d window
    let mut window = Window::new("Kiss3d: cube");
    // Set lights and stick to camera
    window.set_light(Light::StickToCamera);

    // Open messagequeue
    let mq = PosixMq::open("/degree").unwrap();
    let mut counter = 0;
    let mut degrees = 0.0;
    // Create mspc channel to communicate between threads
    let (tx, rx) = mpsc::channel();

    // Create thread which gets degrees from mq
    thread::spawn(move || {
        loop {
            // Send the result of get_degrees to other thread
            match tx.send(get_degrees(&mq)) {
                Err(e) => println!("SenderError: {}", e),
                Ok(_) => (),
            }
            // Sleep to throttle
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Holds two points for every element (To form lines)
    let mut all_points: Vec<(Point, Point)> = Vec::new();
    // Holds degrees of all lines to determine colors
    let mut all_degrees: Vec<f32> = Vec::new();

    while window.render() {
        // get data from other thread async
        match rx.try_recv() {
            Ok(v) => {
                degrees = v;
                // Calculate x, y and add to array
                let x: f32 = (degrees as f32).sin() * 1.0;
                let z: f32 = (degrees as f32).cos() * 1.0;
                all_points.push(((x, 0.0, z), (x, 1.0, z)));
                all_degrees.push(degrees);
            },
            Err(e) => println!("RecvError: {}", e),
        }
        println!("Counter: {}", counter);

        // Got trough whole array and print all the lines
        for i in 0..all_points.len() {
            let a = Point3::new(all_points[i].0.0, all_points[i].0.1, all_points[i].0.2);
            let b = Point3::new(all_points[i].1.0, all_points[i].1.1, all_points[i].1.2);
            // Print in other color according to degree
            if all_degrees[i] >= 0.0 && all_degrees[i] <= 90.0 {
                window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
            }else if all_degrees[i] > 90.0 && all_degrees[i] <= 180.0 {
                window.draw_line(&a, &b, &Point3::new(0.0, 1.0, 0.0));
            }else if all_degrees[i] > 180.0 && all_degrees[i] <= 270.0 {
                window.draw_line(&a, &b, &Point3::new(0.0, 0.0, 1.0));
            }else if all_degrees[i] > 270.0 && all_degrees[i] <= 360.0 {
                window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 1.0));
            }

        }
        counter += 1;
    }
}

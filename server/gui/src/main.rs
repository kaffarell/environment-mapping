use nalgebra::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;
use posixmq::PosixMq;

fn get_degrees(mq: &PosixMq) -> f32 {
    let mut buf = vec![0; mq.attributes().max_msg_len];
    println!("{:?}", mq.receive(&mut buf).unwrap());
    let degrees = buf[0] - 48;
    println!("{}", degrees);
    return degrees as f32;
}

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);
    let mq = PosixMq::open("/degree").unwrap();
    let mut counter = 0;
    let mut degrees = 0.0;
    while window.render() {
        if counter % 10 == 0 {
            degrees = get_degrees(&mq);
            println!("{}", counter);
        }
        println!("hei");
        let x: f32 = (degrees as f32).sin() * 1.0;
        let z: f32 = (degrees as f32).cos() * 1.0;
        let a = Point3::new(x, 0.0, z);
        let b = Point3::new(x, 1.0, z);
        window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
        counter += 1;
    }
}

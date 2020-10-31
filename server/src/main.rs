use nalgebra::Point3;
use kiss3d::window::Window;
use kiss3d::light::Light;

fn main() {
    let mut window = Window::new("Kiss3d: cube");

    window.set_light(Light::StickToCamera);


    while window.render() {
        // y is up
        let a = Point3::new(0.0, 0.0, 0.0);
        let b = Point3::new(0.0, 1.0, 0.0);

        let c = Point3::new(1.0, 0.0, 1.0);
        let d = Point3::new(1.0, 1.0, 1.0);

        let e = Point3::new(-1.0, 0.0, -1.0);
        let f = Point3::new(-1.0, 1.0, -1.0);

        
        window.draw_line(&a, &b, &Point3::new(1.0, 0.0, 0.0));
        window.draw_line(&c, &d, &Point3::new(0.0, 1.0, 0.0));
        window.draw_line(&e, &f, &Point3::new(0.0, 0.0, 1.0));
    }
}

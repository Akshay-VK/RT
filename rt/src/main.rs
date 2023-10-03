use rt_lib::{Engine, camera::Camera, geometry::geometry::Sphere};

fn main() {
    println!("Hello, world!");
    let mut cam=Camera::new(0.0,0.0,0.0,400, 16.0/9.0, 1.0);
    cam.initialize();
    let mut eng= Engine::init(cam.cam_dim.x,cam.cam_dim.y,cam);
    //
    let sphere1=Box::new(Sphere::new((0.0,0.0,-1.0), 0.5));
    let sphere2=Box::new(Sphere::new((0.0,-100.5,-1.0), 100.0));
    
    //
    eng.run();
    eng.save();
}

use rt_lib::{Engine, camera::Camera};

fn main() {
    println!("Hello, world!");
    let mut cam=Camera::new(0.0,0.0,0.0,400, 16.0/9.0, 1.0);
    cam.initialize();
    let mut eng= Engine::init(cam.cam_dim.x,cam.cam_dim.y,cam);
    eng.run();
    eng.save();
}

use rt_lib::{Engine, camera::Camera};

fn main() {
    println!("Hello, world!");
    let cam=Camera::new(0.0,0.0,0.0,100, 16.0/9.0, 1.0);
    let mut eng= Engine::init(256,256,cam);
    eng.run();
    eng.save();
}

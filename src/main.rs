
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

use gl;

mod render;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::GLES);
    gl_attr.set_context_version(3, 0);

    let window = video_subsystem.window("Window", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // Unlike the other example above, nobody created a context for your window, so you need to create one.
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
    debug_assert_eq!(gl_attr.context_version(), (3, 0));

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut color = (0.2, 0.3, 0.4, 1.0);

    'running: loop {
        unsafe {
            gl::ClearColor(color.0, color.1, color.2, color.3);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::R), .. } => color.0 = 0.7,
                Event::KeyDown { keycode: Some(Keycode::G), .. } => color.1 = 0.6,
                Event::KeyDown { keycode: Some(Keycode::B), .. } => color.2 = 0.8,
                Event::KeyUp { keycode: Some(Keycode::R), .. } => color.0 = 0.2,
                Event::KeyUp { keycode: Some(Keycode::G), .. } => color.1 = 0.3,
                Event::KeyUp { keycode: Some(Keycode::B), .. } => color.2 = 0.4,
                _ => {}
            }
        }
        std::thread::sleep(std::time::Duration::from_secs_f32(1./60.));
    }
}
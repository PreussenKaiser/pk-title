use renderer::Renderer;
use sdl2::event::Event;

mod renderer;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let mut renderer = Renderer::new(sdl_context.video()?)?;

    'running: loop {
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        renderer.draw()?;
    }

    Ok(())
}

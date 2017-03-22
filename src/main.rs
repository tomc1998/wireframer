#[macro_use]
extern crate glium;

use glium::backend::glutin_backend::GlutinFacade;

/// Renderer module. Takes care of rendering our data.
mod renderer;

/// View module. This describes a 'view', which is a screen that can contain
/// components. In a full app, you will be able to navigate between views.
mod view;

/// Module to represent wireframe components. These go inside views.
mod component;

/// Canvas mod. Represent this software's central canvas, which renders all of the views.
mod canvas;

use std::rc::Rc;

fn init_display() -> GlutinFacade {
  use glium::DisplayBuild;
  glium::glutin::WindowBuilder::new()
    .with_gl(glium::glutin::GlRequest::Specific(
        glium::glutin::Api::OpenGl, (3, 0)))
    .build_glium().unwrap()
}

fn init_canvas() -> canvas::Canvas {
  let mut canvas = canvas::Canvas::new();
  canvas.add_view(Rc::new(view::View::new(
        String::from("Test view 1"), 800.0, 600.0)), 
    0.0, 0.0);
  canvas.add_view(Rc::new(view::View::new(
        String::from("Test view 2"), 800.0, 600.0)), 
    1000.0, 0.0);
  return canvas;
}


fn main() {
  use glium::Surface;
  let display = init_display();
  let mut renderer = renderer::Renderer::new(&display);
  let canvas = init_canvas();

  loop {
    // Poll events
    for e in display.poll_events() {
      match e {
        glium::glutin::Event::Closed => return,
        _ => continue,
      }
    }

    // Render
    let mut target = display.draw();
    target.clear_color(0.05, 0.05, 0.05, 1.0); 
    renderer.render(&mut target, &canvas);
    let _ = target.finish();
  }
}

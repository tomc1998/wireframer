use glium::glutin::Event;
use glium::glutin::MouseButton;
use glium::glutin::ElementState;
use glium::glutin::MouseScrollDelta;

use renderer::Renderer;

/// Holds input state, like last mouse position.
pub struct InputHandler {
  pub last_mouse_pos: [f32; 2],
  pub mouse_1_down: bool,
  pub mouse_2_down: bool,

  /// Set to true if this is the first frame that input handler has process
  /// input. This is to prevent stuff like last_mouse_pos being set to a bad
  /// value, and a huge delta being created.
  pub first_frame: bool,
}

impl InputHandler {
  pub fn new() -> InputHandler {
    InputHandler {
      last_mouse_pos: [0.0, 0.0],
      mouse_1_down: false,
      mouse_2_down: false,
      first_frame: true,
    }
  }

  /// Handle an input event.
  /// # Params
  /// * `renderer` - The renderer to be mutated by the input event (i.e. for
  ///                mouse scrolling affecting the renderer projection matrix).
  /// * `event`    - The event to process.
  pub fn handle_event(&mut self, renderer: &mut Renderer, e: &Event) {
    match *e {
      Event::MouseMoved(x, y) => {
        let x = x as f32;
        let y = y as f32;
        let delta = (x - self.last_mouse_pos[0], y - self.last_mouse_pos[1]);
        if !self.first_frame {
          if self.mouse_2_down {
            renderer.camera_pos[0] -= delta.0 * renderer.camera_zoom;
            renderer.camera_pos[1] -= delta.1 * renderer.camera_zoom;
          }
        }
        self.last_mouse_pos[0] = x;
        self.last_mouse_pos[1] = y;
      },
      Event::MouseWheel(MouseScrollDelta::LineDelta(x, y), _) => {
        renderer.camera_zoom -= y * 0.1f32;
        if renderer.camera_zoom < 0.1f32 {
          renderer.camera_zoom = 0.1f32;
        }
      },
      Event::MouseInput(state, button) => {
        match button {
          MouseButton::Left => match state {
            ElementState::Pressed => self.mouse_1_down = true,
            ElementState::Released => self.mouse_1_down = false,
          },
          MouseButton::Right => match state {
            ElementState::Pressed => self.mouse_2_down = true,
            ElementState::Released => self.mouse_2_down = false,
          },
          _ => return,
        }
      }
      _ => return,
    }
    self.first_frame = false;
  }
}

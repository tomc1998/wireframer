use view::View;
use std::rc::Weak;

/// A wireframe component.
pub struct Component {
  /// Size of the component in pixels.
  pub size: [f32; 2],

  /// Position of the component in pixels, relative to the top left of the
  /// view this component belongs to.
  pub pos: [f32; 2],

  pub view: Weak<View>,
}

impl Component {
  /// Creates a new Component object.
  /// # Params
  /// * x_pos - The x position of this component in the view, in pixels (origin top left).
  /// * y_pos - The y position of this component in the view, in pixels (origin top left).
  /// * width - The width of this component in pixels.
  /// * height - The height of this component in pixels.
  /// * view - A weak reference to the view this component is being placed into.
  pub fn new(x_pos: f32, y_pos: f32, width: f32, height: f32, view: Weak<View>) -> Component {
    Component {
      pos: [x_pos, y_pos],
      size: [width, height],
      view: view,
    }
  }
}

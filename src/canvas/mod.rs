use view::View;
use std::rc::Rc;

/// Contains data for a view, plus rendering metadata (i.e. position on the
/// canvas).
pub struct ViewOnCanvas {
  pub view: Rc<View>,
  pub canvas_pos: [f32; 2],
}

impl ViewOnCanvas {
  /// Create a new ViewOnCanvas object.
  /// # Params
  /// * view - The view data.
  /// * pos_x - X position on the canvas (origin is top left)
  /// * pos_y - Y position on the canvas (origin is top left)
  pub fn new(view: Rc<View>, pos_x: f32, pos_y: f32) -> ViewOnCanvas {
    ViewOnCanvas {
      view: view,
      canvas_pos: [pos_x, pos_y],
    }
  }
}

pub struct Canvas {
  pub view_list: Vec<ViewOnCanvas>,
}

impl Canvas {
  pub fn new() -> Canvas {
    Canvas {
      view_list: Vec::new(),
    }
  }

  /// Add a view to the canvas.
  /// # Params
  /// * view - The view data.
  /// * pos_x - X position on the canvas (origin is top left)
  /// * pos_y - Y position on the canvas (origin is top left)
  pub fn add_view(&mut self, view: Rc<View>, pos_x: f32, pos_y: f32) {
    self.view_list.push(ViewOnCanvas::new(view, pos_x, pos_y))
  }
}

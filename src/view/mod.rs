use component::Component;
use std::rc::Rc;
use std::cell::RefCell;

pub struct View {
  /// Name of the view. Assigned by the user on creation.
  pub name: String,

  /// Final resolution in pixels of this screen. This is an array of `f32` to
  /// avoid tedious casts everywhere. Realistically it should only be set to
  /// whole values.
  pub resolution: [f32; 2],

  /// List of components that this view contains.
  pub components: RefCell<Vec<Rc<Component>>>
}

impl View {
  /// Create a new view. 
  /// # Params
  /// * name - The name of the view.
  /// * width - The width of the view in pixels. Not necessarily the size it
  ///           will be rendered on the canvas.
  /// * height - The height of the view in pixels. Not necessarily the size it
  ///            will be rendered on the canvas.
  pub fn new(name: String, width: f32, height: f32) -> View {
    View {
      name: name,
      resolution: [width, height],
      components: RefCell::new(Vec::new()),
    }
  }
}


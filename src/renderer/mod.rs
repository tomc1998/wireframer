use glium;
use glium::backend::glutin_backend::GlutinFacade;
use canvas;

#[derive(Clone, Copy)]
struct Vertex {
  /// X, Y, Z
  pos: [f32; 3],
  /// R, G, B, A
  color: [f32; 4],
}
impl Vertex {
  fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32, a: f32) -> Vertex {
    Vertex{ pos: [x, y, z], color: [r, g, b, a] }
  }
}
implement_vertex!(Vertex, pos, color);

pub struct Renderer {
  program: glium::Program,

  /// Handle to the OpenGL vertex buffer
  v_buf: glium::VertexBuffer<Vertex>,

  pub camera_pos: [f32; 2],
}

const VERT_SHADER_SRC : &'static str = r#"
  #version 100

  uniform mat4 proj_mat;

  attribute vec3 pos;
  attribute vec4 color;

  varying vec4 v_color;

  void main() {
    v_color = color;
    gl_Position = proj_mat*vec4(pos, 1.0);
  } "#;

const FRAG_SHADER_SRC : &'static str = r#"
  #version 100
  precision mediump float; // Float precision to medium

  varying vec4 v_color;

  void main() {
    gl_FragColor = v_color;
  }"#;


impl Renderer {
  pub fn new(display: &GlutinFacade) -> Renderer {
    const BUF_SIZE: usize = 2048;
    Renderer {
      program: glium::Program::from_source(display, VERT_SHADER_SRC, FRAG_SHADER_SRC, None).unwrap(),
      v_buf: glium::VertexBuffer::empty_dynamic(display, BUF_SIZE).unwrap(),
      camera_pos: [0.0, 0.0],
    }
  }

  /// Generates vertex data given a reference to a canvas.
  /// # Params
  /// * canvas - The canvas to generate vertex data for. 
  /// # Returns
  /// The vertex data needed to upload to a VBO.
  fn gen_vertex_data_from_canvas(&self, canvas: &canvas::Canvas) -> Vec<Vertex> {
    let mut v_data = Vec::new();
    for v in &canvas.view_list {
      // Add view quad to the vertex list
      v_data.push(Vertex::new(v.canvas_pos[0], 
                              v.canvas_pos[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
      v_data.push(Vertex::new(v.canvas_pos[0] + v.view.resolution[0], 
                              v.canvas_pos[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
      v_data.push(Vertex::new(v.canvas_pos[0] + v.view.resolution[0], 
                              v.canvas_pos[1] + v.view.resolution[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
      v_data.push(Vertex::new(v.canvas_pos[0], 
                              v.canvas_pos[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
      v_data.push(Vertex::new(v.canvas_pos[0],
                              v.canvas_pos[1] + v.view.resolution[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
      v_data.push(Vertex::new(v.canvas_pos[0] + v.view.resolution[0], 
                              v.canvas_pos[1] + v.view.resolution[1], 
                              0.0, 0.9, 0.9, 0.9, 1.0));
    }
    return v_data;
  }

  /// Render a canvas to a given glium target.
  /// # Params
  /// * target - The frame to render to.
  /// * canvas - The canvas data to render.
  pub fn render(&mut self, target: &mut glium::Frame, canvas: &canvas::Canvas) {
    use glium::Surface;
    let data = self.gen_vertex_data_from_canvas(canvas);
    self.v_buf.slice(0 .. data.len()).unwrap().write(&data);

    let (w, h) = target.get_dimensions();
    let w = w as f32;
    let h = h as f32;
    // (right + left)/(right - left)
    let tx = - (w + self.camera_pos[0] + self.camera_pos[0])/(w);
    // - (top + bottom)/(top - bottom)
    let ty = - (h + self.camera_pos[1] + self.camera_pos[1])/(-w);

    let uniforms = uniform! {
      proj_mat: [
        [2.0/w,  0.0,    0.0, 0.0],
        [0.0,   -2.0/h,  0.0, 0.0],
        [0.0,    0.0,   -1.0, 0.0],
        [ tx,     ty,    0.0, 1.0]
      ],
    };

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    target.draw(&self.v_buf, &indices, &self.program, &uniforms,
                &Default::default()).unwrap();
  }
}

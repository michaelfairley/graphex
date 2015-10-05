use cgmath;
use glium;

#[derive(Copy, Clone)]
pub struct Vertex {
  position: [f32; 3],
  normal: [f32; 3],
}

implement_vertex!(Vertex, position, normal);

pub const CUBE: &'static[Vertex] = &[
  // Front
  Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },
  Vertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0,  0.0,  1.0] },

  // Back
  Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },
  Vertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0,  0.0, -1.0] },

  // Left
  Vertex { position: [-0.5,  0.5,  0.5], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-0.5,  0.5, -0.5], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-0.5, -0.5,  0.5], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [-0.5, -0.5, -0.5], normal: [-1.0,  0.0,  0.0] },

  // Right
  Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 1.0,  0.0,  0.0] },

  // Bottom
  Vertex { position: [ 0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [-0.5, -0.5,  0.5], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [-0.5, -0.5, -0.5], normal: [ 0.0, -1.0,  0.0] },

  // Top
  Vertex { position: [ 0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-0.5,  0.5,  0.5], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-0.5,  0.5, -0.5], normal: [ 0.0,  1.0,  0.0] },
  ];

pub const OCTAHEDRON: &'static[Vertex] = &[
  Vertex { position: [ 0.0,  1.0,  0.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 1.0,  0.0,  0.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  0.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

  Vertex { position: [ 1.0,  0.0,  0.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  1.0,  0.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 0.0,  0.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

  Vertex { position: [ 1.0,  0.0,  0.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0, -1.0,  0.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 0.0,  0.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

  Vertex { position: [ 0.0, -1.0,  0.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 1.0,  0.0,  0.0], normal: [ 1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  0.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

  Vertex { position: [-1.0,  0.0,  0.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  1.0,  0.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [ 0.0,  0.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

  Vertex { position: [ 0.0,  1.0,  0.0], normal: [ 0.0,  1.0,  0.0] },
  Vertex { position: [-1.0,  0.0,  0.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  0.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

  Vertex { position: [ 0.0, -1.0,  0.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [-1.0,  0.0,  0.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0,  0.0,  1.0], normal: [ 0.0,  0.0,  1.0] },

  Vertex { position: [-1.0,  0.0,  0.0], normal: [-1.0,  0.0,  0.0] },
  Vertex { position: [ 0.0, -1.0,  0.0], normal: [ 0.0, -1.0,  0.0] },
  Vertex { position: [ 0.0,  0.0, -1.0], normal: [ 0.0,  0.0, -1.0] },

];

pub struct SolidEntity<'b> {
  pub buffer: &'b glium::VertexBuffer<Vertex>,
  pub color: [f32; 3],
  pub matrix: cgmath::Matrix4<f32>,
}

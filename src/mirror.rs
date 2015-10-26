use shapes;
use glium;
use cgmath;

pub struct Mirror<'b> {
  pub buffer: &'b glium::VertexBuffer<shapes::Vertex>,
  pub color: [f32; 3],
  pub position: cgmath::Vector3<f32>,
  pub rotation: cgmath::Matrix3<f32>,
}

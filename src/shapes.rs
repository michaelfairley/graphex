use cgmath;
use cgmath::{Vector,Vector3,vec3,EuclideanVector,FixedArray};
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

type Triangle = [Vector3<f32>; 3];

fn triforce_and_normalize(triangles: &Vec<Triangle>) -> Vec<Triangle> {
  triangles.iter().flat_map( |t| {
    let a = t[0];
    let b = t[1];
    let c = t[2];
    let ab = a.add_v(&b).div_s(2.0).normalize();
    let bc = b.add_v(&c).div_s(2.0).normalize();
    let ca = c.add_v(&a).div_s(2.0).normalize();

    vec![
      [a, ab, ca],
      [b, bc, ab],
      [c, ca, bc],
      [ab, bc, ca],
      ]
  }).collect::<Vec<Triangle>>()
}

lazy_static! {
  static ref OCTAHEDRON_NORMALS: Vec<Vector3<f32>> = vec![
    vec3( 1.0,  1.0,  1.0),
    vec3( 1.0,  1.0, -1.0),
    vec3( 1.0, -1.0,  1.0),
    vec3( 1.0, -1.0, -1.0),
    vec3(-1.0,  1.0,  1.0),
    vec3(-1.0,  1.0, -1.0),
    vec3(-1.0, -1.0,  1.0),
    vec3(-1.0, -1.0, -1.0),
    ];

  static ref OCTAHEDRON_TRIANGLES: Vec<Triangle> =
    OCTAHEDRON_NORMALS.iter()
    .map( |n|
           if n.x * n.y * n.z > 0.0 {
             [
               vec3(0.0, n.y, 0.0),
               vec3(n.x, 0.0, 0.0),
               vec3(0.0, 0.0, n.z),
               ]
           } else {
             [
               vec3(n.x, 0.0, 0.0),
               vec3(0.0, n.y, 0.0),
               vec3(0.0, 0.0, n.z),
               ]
           }
           ).collect::<Vec<Triangle>>();
}

pub fn sphere(iterations: u32) -> Vec<Vertex> {
  let mut triangles: Vec<Triangle> = triforce_and_normalize(&OCTAHEDRON_TRIANGLES);

  for _ in (1..iterations) {
    triangles = triforce_and_normalize(&triangles);
  };

  triangles.iter().flat_map( |t|
                vec![
                  Vertex { position: t[0].into_fixed(), normal: t[0].into_fixed() },
                  Vertex { position: t[1].into_fixed(), normal: t[1].into_fixed() },
                  Vertex { position: t[2].into_fixed(), normal: t[2].into_fixed() },
                  ]
                ).collect::<Vec<Vertex>>()
}

pub enum Shading {
  Flat,
  Smooth
}

pub struct SolidEntity<'b> {
  pub buffer: &'b glium::VertexBuffer<Vertex>,
  pub color: [f32; 3],
  pub matrix: cgmath::Matrix4<f32>,
  pub shading: Shading,
}

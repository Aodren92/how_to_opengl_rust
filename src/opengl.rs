pub mod triangle;

pub type VERTEX = [f32; 3];

pub fn draw_simple_triangle() -> usize {

    let vertices: [VERTEX; 3] = [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];
    triangle::draw_triangle(vertices, None);
    return vertices.len();
}

pub fn draw_simple_rectangle() -> usize {

   let vertices: [VERTEX; 6] = [
           // first triangle
       [ 0.5,  0.5,  0.0],  // top right
       [ 0.5, -0.5,  0.0],  // bottom right
       [-0.5,  0.5,  0.0],  // top left 
       // second triangle
       [ 0.5, -0.5, 0.0],  // bottom right
       [-0.5, -0.5, 0.0],  // bottom left
       [-0.5,  0.5, 0.0]   // top left
   ];
   triangle::draw_triangle(vertices, None);
   return vertices.len();
}

pub fn draw_simple_rectangle_with_indices() -> usize {

   let vertices: [VERTEX; 4] = [
       [ 0.5,  0.5,  0.0],  // top right
       [ 0.5, -0.5,  0.0],  // bottom right
       [-0.5, -0.5,  0.0],  // bottom left
       [-0.5,  0.5,  0.0]   // top left
   ];

   let indices: [u32; 6] = [
       0, 1, 3,
       1, 2, 3
   ];
   triangle::draw_triangle(vertices, Some(&indices));
   return indices.len();
}

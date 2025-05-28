pub mod triangle;

pub type VERTEX = [f32; 3];

pub fn draw_simple_triangle() -> usize {

    let vertices: [VERTEX; 3] = [
        [-0.5, -0.5, 0.0],
        [0.5, -0.5, 0.0],
        [0.0, 0.5, 0.0],
    ];
    triangle::draw_triangle(vertices);
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
   triangle::draw_triangle(vertices);
   return vertices.len();
}

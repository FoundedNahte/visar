
use std::{thread, time};
pub use super::*;
use rand::{thread_rng, Rng};

pub fn create_rand_array(size: u16) -> Vec<u16> {
    let mut arr = Vec::<u16>::new();
    for i in 0..size {
        arr.push((i as u16) + 1);
    }
    let mut rng = thread_rng();
    for i in 0..size {
        let s: usize = rng.gen_range(0..size-1).into();
        let temp = arr[s];
        arr[s] = arr[i as usize];
        arr[s] = arr[i as usize];
        arr[i as usize] = temp;
    }
    arr
}
pub struct VisualArray {
    pub array: Vec<u16>,
    pub size: usize,
    pub vertices: Vec<surface::Vertex>,
    pub indices: Vec<u16>,
}

pub enum Color {
    White,
    Red,
    Green,
}

impl VisualArray{
    pub fn new(arr: Vec<u16>) -> VisualArray {
        let mut vertices = Vec::<surface::Vertex>::new();
        //let mut Vertices = [surface::Vertex; 4*arr.len()];
        //let mut Indices = [u16; 6*arr.len()];
        let mut indices = Vec::<u16>::new();
        let mut index = 0;
        // divding the area into equal number of spaces for rectangles
        let mut x_position = 0.90;
        let mut x_drawing_area = 1.8;
        let y_drawing_area = 1.5;
        x_drawing_area = x_drawing_area - (((arr.len()-1) as f32)*(0.001/(arr.len() as f32)));
        let x_delta = x_drawing_area/(arr.len() as f32);
        let y_delta = y_drawing_area/(arr.len() as f32);
        for rect_height in arr.iter() {
            // Vertices for the left side triangle (upright triangle)
            let vertex_left = {
                vertices.push(surface::Vertex { position: [-x_position, -0.95, 0.0], color: [1.0, 1.0, 1.0] }); 
                index+=1;
                index-1
            };
            let vertex_top = {
                vertices.push(surface::Vertex { position: [-x_position, (-0.95+(y_delta*(*rect_height as f32))), 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };
            let vertex_right = {
                vertices.push(surface::Vertex { position: [-(x_position+x_delta), -0.95, 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };
            // We already know two of the three vertices for the inverted triangle, so we only
            // create one more
            let vertex_top_right = {
                vertices.push(surface::Vertex { position: [-(x_position+x_delta), (-0.95+(y_delta*(*rect_height as f32))), 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };

            indices.push(vertex_left);
            indices.push(vertex_top);
            indices.push(vertex_right);
            indices.push(vertex_top);
            indices.push(vertex_top_right);
            indices.push(vertex_right);


            x_position -= x_delta+(0.01/(arr.len() as f32));
        }
        
        let size = arr.len();

        VisualArray {
            array: arr,
            size: size,
            vertices: vertices,
            indices: indices,
        }
    }
    
    pub fn update(&mut self, arr: Vec<u16>) {
        let mut vertices = Vec::<surface::Vertex>::new();

        let mut indices = Vec::<u16>::new();
        let mut index = 0;
        let mut x_position = 0.90;
        let mut x_drawing_area = 1.8;
        let y_drawing_area = 1.5;
        x_drawing_area = x_drawing_area - (((arr.len()-1) as f32)*(0.001/(arr.len() as f32)));
        let x_delta = x_drawing_area/(arr.len() as f32);
        let y_delta = y_drawing_area/(arr.len() as f32);
        for rect_height in arr.iter() {
            let vertex_left = {
                vertices.push(surface::Vertex { position: [-x_position, -0.95, 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };
            let vertex_top = {
                vertices.push(surface::Vertex { position: [-x_position, (-0.95+(y_delta*(*rect_height as f32))), 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };
            let vertex_right = {
                vertices.push(surface::Vertex { position: [-(x_position+x_delta), -0.95, 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };
            // We already know two of the three vertices for the inverted triangle, so we only
            // create one more
            let vertex_top_right = {
                vertices.push(surface::Vertex { position: [-(x_position+x_delta), (-0.95+(y_delta*(*rect_height as f32))), 0.0], color: [1.0, 1.0, 1.0] });
                index+=1;
                index-1
            };

            indices.push(vertex_left);
            indices.push(vertex_top);
            indices.push(vertex_right);
            indices.push(vertex_top);
            indices.push(vertex_top_right);
            indices.push(vertex_right);


            x_position -= x_delta+(0.01/(arr.len() as f32));
        }

        self.vertices = vertices;
        self.indices = indices;
        self.array = arr;
    }

    pub fn change_color(&mut self, state: &mut surface::State, index_1: u32, rect_color: Color) {
        let vertices = &mut self.vertices;

        let arr;
        match rect_color {
            Color::White => { arr = [1.0, 1.0, 1.0] },
            Color::Red => { arr = [1.0, 0.0, 0.0] },
            Color::Green => { arr = [0.0, 1.0, 0.0] },
        }
            
        vertices[self.indices[(((index_1+1)*6)-6) as usize] as usize].color = arr;
        vertices[self.indices[(((index_1+1)*6)-5) as usize] as usize].color = arr;
        vertices[self.indices[(((index_1+1)*6)-4) as usize] as usize].color = arr;
        vertices[self.indices[(((index_1+1)*6)-2) as usize] as usize].color = arr;
        
        state.update(&self.vertices, &self.indices);
        
    }

    pub fn access(&mut self, index: usize) -> u16 {
        self.array[index]
    }
    
    pub fn change(&mut self, index: usize, value: u16) {
        self.array[index] = value;
    }

    pub fn swap(&mut self, state: &mut surface::State, index_1: u32, index_2: u32) {
        //change color of first rectangle to indicate that it is being selected
        {
            // 1 millisecond delay to allow user to see visualization
            //let one_milli = time::Duration::from_millis(1);
            //thread::sleep(one_milli);
            // indices of each rectangle are stored as 6 indices to the certain vertices in the
            // vertices array
            
            // We have to switch the x positions of each rectangle
            //Switch rectangle 1
            self.change_color(state, index_1, Color::Red);


            {
                let vertices = &mut self.vertices;
                let rect1_height = vertices[self.indices[(((index_1+1)*6)-2) as usize] as usize].position[1];
                // get values for rect2
                let rect2_height = vertices[self.indices[(((index_2+1)*6)-2) as usize] as usize].position[1];
                
                vertices[self.indices[(((index_1+1)*6)-2) as usize] as usize].position[1] = rect2_height;
                vertices[self.indices[(((index_1+1)*6)-3) as usize] as usize].position[1] = rect2_height; 

                vertices[self.indices[(((index_2+1)*6)-2) as usize] as usize].position[1] = rect1_height;     
                vertices[self.indices[(((index_2+1)*6)-3) as usize] as usize].position[1] = rect1_height;
            }

            self.change_color(state, index_1, Color::White);
            /*
            VERTICES[self.indices[(((index_1+1)*6)-6) as usize] as usize].color = [1.0, 1.0, 1.0];
            VERTICES[self.indices[(((index_1+1)*6)-5) as usize] as usize].color = [1.0, 1.0, 1.0];
            VERTICES[self.indices[(((index_1+1)*6)-4) as usize] as usize].color = [1.0, 1.0, 1.0];
            VERTICES[self.indices[(((index_1+1)*6)-2) as usize] as usize].color = [1.0, 1.0, 1.0];
            */
        }

        state.update(&self.vertices, &self.indices);
    }

    pub fn finish(&mut self, state: &mut surface::State) {
        for i in 0..self.size {
            self.change_color(state, i as u32, Color::Green);
        }

        for i in 0..self.size {
            self.change_color(state, i as u32, Color::White);
        }
    }
}
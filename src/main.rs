use random_triangles::*;
use image::{ImageBuffer, RgbImage};

const X_SIZE  : u32 = 10000;
const Y_SIZE  : u32 = 10000;
const X_PIXEL : u32 = X_SIZE + 1;
const Y_PIXEL : u32 = Y_SIZE + 1;

const X_SCALE : f64 = X_SIZE as f64 / 10.;
const Y_SCALE : f64 = Y_SIZE as f64 / 10.;

const RECURSION_DEPTH : u32 = 13;

fn main()
{
    let (a, b, c) = get_default_triangle();

    let mut new_triangle_1 : MyTriangle;
    let mut new_triangle_2 : MyTriangle;
    let mut new_triangle_3 : MyTriangle;

    let mut triangle : MyTriangle = MyTriangle
    {
        left  : a.clone(),
        right : b.clone(),
        top   : c.clone(),
    };

    let mut vec_triangle     : Vec<MyTriangle> = vec![];
    let mut vec_triangle_new : Vec<MyTriangle> = vec![];
    
    
    vec_triangle.push(triangle);

    for _ in 0..RECURSION_DEPTH
    {
        while vec_triangle.len() > 0
        {
            triangle = vec_triangle.pop().unwrap();
            (new_triangle_1, new_triangle_2, new_triangle_3) = triangle.create_sub_triangles();
            vec_triangle_new.push(new_triangle_1);
            vec_triangle_new.push(new_triangle_2);
            vec_triangle_new.push(new_triangle_3);
        };
        vec_triangle.append(&mut vec_triangle_new);
    };

    


    let mut image: RgbImage = ImageBuffer::new(X_PIXEL, Y_PIXEL);

    // for point in vec_points.iter()
    let mut x : f64;
    let mut y : f64;
    for triangle in vec_triangle
    {
        x = triangle.left.x;
        y = triangle.left.y;
        let scaled_x : u32 = (x * X_SCALE).round() as u32;
        let scaled_y : u32 = (y * Y_SCALE).round() as u32;
        *image.get_pixel_mut(scaled_x, scaled_y) = image::Rgb([255, 255, 255]);

        x = triangle.right.x;
        y = triangle.right.y;
        let scaled_x : u32 = (x * X_SCALE).round() as u32;
        let scaled_y : u32 = (y * Y_SCALE).round() as u32;
        *image.get_pixel_mut(scaled_x, scaled_y) = image::Rgb([255, 255, 255]);

        x = triangle.top.x;
        y = triangle.top.y;
        let scaled_x : u32 = (x * X_SCALE).round() as u32;
        let scaled_y : u32 = (y * Y_SCALE).round() as u32;
        *image.get_pixel_mut(scaled_x, scaled_y) = image::Rgb([255, 255, 255]);
            
    };

    image.save("output1.png").unwrap();
    // println!("{:?}", vec_points);
}

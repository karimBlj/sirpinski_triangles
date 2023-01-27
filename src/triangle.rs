use crate::*;

pub struct MyTriangle
{
    pub left  : Point,
    pub right : Point,
    pub top   : Point,
}

impl MyTriangle
{
    pub fn create_sub_triangles
    (
        &self
    ) -> (MyTriangle, MyTriangle, MyTriangle)
    {
        let new_point_left  : Point = middle_point(&self.left,  &self.top );
        let new_point_right : Point = middle_point(&self.right, &self.top );
        let new_point_bot   : Point = middle_point(&self.right, &self.left);

        let new_triangle_1 : MyTriangle = MyTriangle
        {
            left  : self.left .clone(),
            right : new_point_bot .clone(),
            top   : new_point_left.clone()
        };

        let new_triangle_2 : MyTriangle = MyTriangle
        {
            left  : new_point_bot  .clone(),
            right : self.right .clone(),
            top   : new_point_right.clone(),
        };

        let new_triangle_3 : MyTriangle = MyTriangle
        {
            left  : new_point_left .clone(),
            right : new_point_right.clone(),
            top   : self.top   .clone(),
        };

        return (new_triangle_1, new_triangle_2, new_triangle_3);
    }
}
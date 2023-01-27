use rand::Rng;
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Point
{
    pub x : f64,
    pub y : f64,
}

pub fn middle_point
(
    point1 : &Point,
    point2 : &Point,
) -> Point
{
    // let midle_x = max(point1.x, point2.x) - min(point1.x, point2.x);
    // let midle_y = max(point1.y, point2.y) - min(point1.y, point2.y);
    let midle_x = (point1.x + point2.x) * 0.5;
    let midle_y = (point1.y + point2.y) * 0.5;

    return Point
    {
        x : midle_x,
        y : midle_y
    };
}

pub fn get_default_triangle() -> (Point, Point, Point)
{
    let a = Point
    {
        x : 0.,
        y : 0.,
    };
    let b = Point
    {
        x : 10.,
        y : 0.,
    };
    let c = Point
    {
        x : 5.,
        y : 5 as f64 * (3 as f64).sqrt(),
    };

    return (a, b, c);
}

pub fn point_in_default_triangle
(
    point : &Point
) -> bool
{
    let (a, b, c) = get_default_triangle();

    let line1 = get_linear_coefficient(&a, &c);
    let line2 = get_linear_coefficient(&c, &b);

    let y_line_1 = compute_point_value(point.x, &line1);
    let y_line_2 = compute_point_value(point.x, &line2);
    if point.y > y_line_1
    {
        return false;
    }
    if point.y > y_line_2
    {
        return false;
    }
    if point.y < 0.
    {
        return false;
    }

    return true;
}

fn compute_point_value
(
    x          : f64,
    line_coeff : &Point,
) -> f64
{
    return line_coeff.x * x + line_coeff.y;
}

fn get_linear_coefficient
(
    point1 : &Point,
    point2 : &Point,
) -> Point
{
    let delta_x = point2.x - point1.x;
    let delta_y = point2.y - point1.y;

    let a = delta_y / delta_x;
    let b = point1.y - a * point1.x;

    return Point
    {
        x : a,
        y : b,
    };
}

fn get_random_xy() -> (f64, f64)
{
    let max_x = 10.;
    let max_y = 5. * (3 as f64).sqrt();
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(0.0..max_x);
    let y = rng.gen_range(0.0..max_y);
    return (x, y);
}

pub fn get_random_valid_point() -> Point
{
    let (mut x, mut y) = get_random_xy();
    let mut point = Point{x, y};
    
    while !point_in_default_triangle(&point)
    {
        (x, y) = get_random_xy();
        point = Point{x, y};
    };

    return point;
}
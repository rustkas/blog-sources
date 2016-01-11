extern crate image;
use std::fs::File;
use std::path::Path;
use image::ImageBuffer;

fn main() {
    let begin_point = Point::new(2000, 2000);
    let end_point = Point::new(3000, 2000);
    let begin_color= Color::new(255, 0, 0);
    let end_color = Color::new(0, 255, 255);
    let iterate_count = 25;
    let lines = get_lines(begin_point, begin_color, end_point, end_color, iterate_count);
    create_image(5000, 3000, &lines, "img.png");
}

#[derive(Clone)]
struct Color {
    r:u32,
    g:u32,
    b:u32,
}
impl Color {
    fn new(r:u32, g:u32, b:u32) -> Color
    {
        Color {r: r, g: g, b:b}
    }
}

#[derive(Clone)]
struct Point {
    x:i32,
    y:i32,
}
impl Point {
    fn new(x:i32, y:i32) -> Point
    {
        Point {x: x, y: y}
    }
}

#[derive(Clone)]
struct Line {
    begin_point: Point,
    end_point: Point,
    color: Color,
}


/// Generates PNG image and saves it.
fn create_image(width:u32, height:u32, lines:&Vec<Line>, file_name: &str)
{
    let mut img = ImageBuffer::new(width, height);
    for line in lines {
        draw_line(&mut img, &line.begin_point, &line.end_point, &line.color);
    }
    let ref mut fout = File::create(&Path::new(file_name)).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}

/// Draws line on canvas.
fn draw_line(canvas: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, point_1: &Point, point_2: &Point, color: &Color, )
{
    let px = image::Rgb([color.r as u8, color.g as u8, color.b as u8]);

    let mut dx= point_2.x - point_1.x;
    let mut dy = point_2.y - point_1.y;
    let incx = get_sign(dx);
    let incy = get_sign(dy);
    if dx < 0 {
        dx = - dx;
    }
    if dy < 0 {
        dy = -dy;
    }

    let (pdx, pdy, es, el) =
        if dx > dy {
            (incx, 0, dy, dx)
        } else {
    	    (0, incy, dx, dy)
        };

    let mut x = point_1.x;
    let mut y = point_1.y;
    let mut err = el / 2;

    canvas.put_pixel(x as u32, y as u32, px);

    for _ in 0..el {
        err = err - es;
        if err < 0 {
            err = err + el;
            x = x + incx;
            y = y + incy;
        } else{
            x = x+ pdx;
            y = y + pdy;
        }
        canvas.put_pixel(x as u32, y as u32, px);
    }
}

fn get_sign(value:i32) -> i32
{
    if value > 0 { 1 }
    else if value < 0 { -1 }
    else { 0 }
}

/// Returns list of lines for drawing.
fn get_lines(point_1:Point, color_1:Color, point_2:Point, color_2:Color, level:i32) -> Vec<Line>
{
    let middle_color: Color = get_midle_color(&color_1, &color_2);

    if level == 1 {
        vec![Line {
            begin_point: point_1,
            end_point: point_2,
            color: middle_color,
        }]
    } else {
        let middle_point = get_middle_point(&point_1, &point_2);
        let mut lines: Vec<Line> = vec![];
        let prev = &mut get_lines(point_1, color_1, middle_point.clone(), middle_color.clone(), level - 1);
        let next = &mut get_lines(middle_point, middle_color, point_2, color_2, level - 1);
        lines.append(prev);
        lines.append(next);
        lines
    }
}

/// Returns color placed between color_1 and color_2.
fn get_midle_color(color_1:&Color, color_2:&Color) -> Color
{
    Color {
        r: (color_1.r + color_2.r) / 2,
        g: (color_1.g + color_2.g) / 2,
        b: (color_1.b + color_2.b) / 2,
    }
}

/// Returns point placed between point_1 and point_2.
fn get_middle_point (point_1:&Point, point_2:&Point) -> Point
{
    Point {
        x: (point_1.x + point_2.x) / 2 + (point_2.y - point_1.y) / 2,
        y: (point_1.y + point_2.y) / 2 - (point_2.x - point_1.x) / 2,
    }
}

// Module to draw shapes

#[derive(Debug)]
pub struct VisualizePolicy {
    pub begin_gap : u32,
    pub end_gap   : u32
}

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

fn draw_line_internal(canvas: &mut Vec<String>, x: &Point, y: &Point) {
    // x is the starting point and y is the ending point or offset
    // We have something like 2D Canvas, we can easily render lines to it, but we gotta select nice characters for that
    // Visualize calls on the right side and jumps on the left side
    
}

fn draw_rectangle(canvas: &mut Vec<String>, d1: &Point, d2: &Point) {
}

// Module to draw shapes

#[derive(Debug)]
pub struct VisualizePolicy {
    pub begin_gap: u32,
    pub end_gap: u32,
}

// (x,y) represent mathematical co-ordinates (x,y)
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn draw_line_internal(canvas: &mut Vec<String>, p1: &Point, p2: &Point, lane: u32) {
    // x is the starting point and y is the ending point or offset
    // We have something like 2D Canvas, we can easily render lines to it, but we gotta select nice characters for that
    // Visualize calls on the right side and jumps on the left side
    let left_line: u32 = 5;
    let begin_gap: u32 = 20;
    let tab_length: u32 = 5;

    assert!(lane < 25);

    unsafe {
        // Draw the horizontal line and the arrowhead
        // The first one is the arrowhead (outward line)
        canvas[p1.y as usize].as_mut_vec()[lane as usize] = b'<';
        for i in lane + 1..begin_gap + tab_length - 5 {
            canvas[p1.y as usize].as_mut_vec()[i as usize] = b'-';
        }

        // Draw the vertical line on the lane exactly
        let mut a = p1.y;
        let mut b = p2.y;

        if p2.y < p1.y {
            std::mem::swap(&mut a, &mut b);
        }
        for i in a + 1..b + 1 {
            canvas[i as usize].as_mut_vec()[lane as usize] = b'|';
        }
        // canvas[(p2.y - 1) as usize].as_mut_vec()[lane as usize] = b'v';
        // Finally draw the line pointing inward
        canvas[p2.y as usize].as_mut_vec()[(begin_gap + tab_length - 7) as usize] = b'>';
        for i in lane + 1..begin_gap + tab_length - 7 {
            canvas[p2.y as usize].as_mut_vec()[i as usize] = b'-';
        }
    }
}

fn draw_rectangle(canvas: &mut Vec<String>, d1: &Point, d2: &Point) {}

fn visualize_jumps() {}

// Returns what this function thinks that lane will look nice

fn does_overlap(mut p0: Point, mut p1: Point) -> bool {
    // sort the points
    if p0.y < p0.x {
        std::mem::swap(&mut p0.x, &mut p0.y);
    }

    if p1.y < p1.x {
        std::mem::swap(&mut p1.x, &mut p1.y);
    }

    p0.x < p1.y && p0.y > p1.x
}

enum OverlapInfo {
    None,
    FirstIsInside,
    LatterIsInside,
}

fn is_completely_inside(mut p0: Point, mut p1: Point) -> OverlapInfo {
    if p0.y < p0.x {
        std::mem::swap(&mut p0.x, &mut p0.y);
    }

    if p1.y < p1.x {
        std::mem::swap(&mut p1.x, &mut p1.y);
    }

    if p0.x < p1.x && p0.y < p1.y {
        return OverlapInfo::FirstIsInside;
    } else if p1.x < p0.x && p1.y < p0.y {
        return OverlapInfo::LatterIsInside;
    } else {
        return OverlapInfo::None;
    }
}

fn determine_optimal_lanes(lines: Vec<Point>) -> Vec<u32> {
    // So we have around 20 gaps before the text section actually happens for now
    // So there are possibly 10 free lines at the start
    // Check if any two lanes overlap and maintain their stack
    let mut final_lane = Vec::with_capacity(lines.len()); // Single allocation

    // Assume that the first jump will take the 2nd lane, we have
    let mut current_lane = 16;
    final_lane.resize(lines.len(), current_lane);

    let lane_gap = 4;
    let mut i: usize = 0;

    while i < lines.len() {
        final_lane[i] = current_lane;
        current_lane = 16;

        // check with the previous lap and decide if one lies completely inside of another
        let mut j: usize = 0;
        while j < i {
            if does_overlap(lines[j], lines[i]) {
                let res = is_completely_inside(lines[i], lines[j]);
                match res {
                    OverlapInfo::FirstIsInside => {
                        if final_lane[i] <= final_lane[j] {
                            final_lane[i] = final_lane[j] + lane_gap;
                        }
                        // else leave as it is
                    }
                    OverlapInfo::LatterIsInside => {
                        if final_lane[j] <= final_lane[i] {
                            final_lane[j] = final_lane[i] + lane_gap;
                        }
                    }
                    OverlapInfo::None => {
                        // Determine the larger one
                        let del1 = lines[i].y - lines[i].x;
                        let del2 = lines[j].y - lines[j].x;

                        if del1 > del2 {
                            final_lane[i] = final_lane[j] + lane_gap;
                        } else {
                            final_lane[j] = final_lane[i] + lane_gap;
                        }
                    }
                }
            } else {
            }
            j = j + 1;
        }
        i = i + 1;
    }
    final_lane
}

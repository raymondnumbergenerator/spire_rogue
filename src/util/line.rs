fn line_low(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = Vec::new();

    let dx = x2 - x1;
    let mut dy = y2 - y1;
    let mut yi = 1;

    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = 2 * dy - dx;
    let mut y = y1;

    for x in x1 ..= x2 {
        line.push((x, y));
        if d > 0 {
            y = y + yi;
            d = d - 2 * dx;
        }
        d = d + 2 * dy;
    }

    line
}

fn line_high(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = Vec::new();

    let mut dx = x2 - x1;
    let dy = y2 - y1;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = 2 * dx - dy;
    let mut x = x1;

    for y in y1 ..= y2 {
        line.push((x, y));
        if d > 0 {
            x = x + xi;
            d = d - 2 * dy;
        }
        d = d + 2 * dx;
    }

    line
}

/// Draws a straight line between two points
pub fn line(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<(i32, i32)> {
    let mut line: Vec<(i32, i32)> = Vec::new();

    if (y2 - y1).abs() < (x2 - x1).abs() {
        if x1 > x2 {
            for (x, y) in line_low(x2, y2, x1, y1) { line.push((x, y)); }
        } else {
            for (x, y) in line_low(x1, y1, x2, y2) { line.push((x, y)); }
        }
    } else {
        if y1 > y2 {
            for (x, y) in line_high(x2, y2, x1, y1) { line.push((x, y)); }
        } else {
            for (x, y) in line_high(x1, y1, x2, y2) { line.push((x, y)); }
        }
    }

    line
}
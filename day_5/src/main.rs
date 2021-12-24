use std::cmp::max;
use std::io;
use std::io::BufRead;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line(Point, Point);

impl Line {
    fn is_straight(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn inside_line(&self, point: Point) -> bool {
        if self.0.x == self.1.x {
            return point.x == self.0.x
                && point.y >= i32::min(self.0.y, self.1.y)
                && point.y <= i32::max(self.0.y, self.1.y);
        } else if self.0.y == self.1.y {
            return point.y == self.0.y
                && point.x >= i32::min(self.0.x, self.1.x)
                && point.x <= i32::max(self.0.x, self.1.x);
        } else {
            let start = if self.0.x < self.1.x { self.0 } else { self.1 };
            let end = if self.0.x < self.1.x { self.1 } else { self.0 };
            return point.y == start.y + (end.y - start.y) / (end.x - start.x) * (point.x - start.x)
                && point.x >= start.x
                && point.x <= end.x;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let text_lines = stdin.lock().lines();
    let mut lines = Vec::new();
    let mut max_x: i32 = 0;
    let mut max_y: i32 = 0;

    for text_line in text_lines {
        let parsed = text_line
            .unwrap()
            .split(" -> ")
            .map(|c| {
                c.split(",")
                    .map(|i| i.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let line = Line(
            Point {
                x: parsed[0][0],
                y: parsed[0][1],
            },
            Point {
                x: parsed[1][0],
                y: parsed[1][1],
            },
        );
        max_x = max(max_x, max(line.0.x, line.1.x));
        max_y = max(max_y, max(line.0.y, line.1.y));
        lines.push(line);
    }

    //lines.retain(|l| l.is_straight());
    //dbg!(&lines);

    let mut points_with_overlap = 0;

    for y in 0..max_x + 1 {
        for x in 0..max_y + 1 {
            let mut lines_at_point = 0;
            for line in &lines {
                if line.inside_line(Point { x, y }) {
                    lines_at_point += 1;
                }
            }

            if lines_at_point >= 2 {
                points_with_overlap += 1;
            }

            //print!{"{}", lines_at_point};
        }
        //println!{};
    }

    dbg!(points_with_overlap);
}

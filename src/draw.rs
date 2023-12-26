use minifb::{Key, Window, WindowOptions, KeyRepeat};
use std::cmp::min;
use crate::DirInfo;
use crate::random;
use std::time::Instant;


use crate::random::generate_random_hex_color;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;


struct Line {}

impl Line {

    pub fn draw_line(start: &Vector2, end: &Vector2, color: u32, buffer: &mut Vec<u32>, buffer_width: usize) {
        let x0 = min(start.x, WIDTH - 1);
        let y0 = min(start.y, HEIGHT - 1);
        let x1 = min(end.x, WIDTH - 1);
        let y1 = min(end.y, HEIGHT - 1);

        let dx = (x1 as isize - x0 as isize).abs();
        let dy = (y1 as isize - y0 as isize).abs();

        let sx: i32 = if x0 < x1 { 1 } else { -1 };
        let sy: i32 = if y0 < y1 { 1 } else { -1 };

        let mut err = dx - dy; 
        let mut x = x0;
        let mut y = y0;

        while x != x1 || y != y1 {
            let index = y * buffer_width + x;
            buffer[index] = color;

            let e2 = 2 * err;

            if e2 >= -dy {
                if x == x1 {
                    break;
                }
                err = err - dy;
                x = (x as i32 + sx) as usize;
            }

            if e2 <= dx {
                if y == y1 {
                    break;
                }
                err = err + dx;
                y = (y as i32 + sy) as usize;
            }
        }
    }
}

struct Vector2 {
    x: usize,
    y: usize,
    color: u32
}

impl Vector2 {
    fn draw_vec(self, buffer: &mut Vec<u32>, buffer_width: usize)  {
        let x = min(self.x, WIDTH -1);
        let y = min(self.y, HEIGHT -1);
        let index = y * buffer_width + x;
        buffer[index] = self.color
    }
}
#[derive(Debug, Clone)]
struct Rec2 {
    name: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
}

impl Rec2 {

    pub fn draw_rect(rec: &Rec2, buffer: &mut Vec<u32>, buffer_width: usize) {
        for row in rec.y..min(rec.y + rec.height, HEIGHT) {
            for col in rec.x..min(rec.x + rec.width, buffer_width) {
                let index = row * buffer_width + col;
                buffer[index] = rec.color;
            }
        }
    }

    pub fn draw_rect_with_border(rec: &Rec2, buffer: &mut Vec<u32>, buffer_width: usize, border_color: u32, border_size: usize) {
        // Draw filled rectangle
        for row in rec.y..min(rec.y + rec.height, HEIGHT) {
            for col in rec.x..min(rec.x + rec.width, buffer_width) {
                let index = row * buffer_width + col;
                buffer[index] = rec.color;
            }
        }

        // Draw top border
        for row in rec.y..rec.y + border_size {
            for col in rec.x..min(rec.x + rec.width, buffer_width) {
                let index = row * buffer_width + col;
                buffer[index] = border_color;
            }
        }

        // Draw bottom border
        for row in rec.y + rec.height - border_size..rec.y + rec.height {
            for col in rec.x..min(rec.x + rec.width, buffer_width) {
                let index = row * buffer_width + col;
                buffer[index] = border_color;
            }
        }

        // Draw left border
        for row in rec.y..min(rec.y + rec.height, HEIGHT) {
            for col in rec.x..rec.x + border_size {
                let index = row * buffer_width + col;
                buffer[index] = border_color;
            }
        }

        // Draw right border
        for row in rec.y..min(rec.y + rec.height, HEIGHT) {
            for col in rec.x + rec.width - border_size..rec.x + rec.width {
                let index = row * buffer_width + col;
                buffer[index] = border_color;
            }
        }
    }
}



pub fn noise(dir_info: DirInfo) {
    // Create a window
    let mut window = Window::new(
        "Rectangle Example",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(33200)));

    // Create a buffer to store pixel data
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut rec_vec = Vec::new();
    treemap_recursive(&dir_info, (WIDTH / 2 ) - (600 / 2), (HEIGHT / 2) - (400 / 2 ), 600, 400, &mut rec_vec);
    println!("{}", rec_vec.len());
    let mut step = 0;
    let MAX_STEP: usize = rec_vec.len() -1;
    const MIN_STEP :usize = 0;
    let flag = true; // set this to false for forward and backward step DEBUGGING
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        if flag {
            buffer.iter_mut().for_each(|pixel| *pixel = 0);
            for rec in &rec_vec {
                Rec2::draw_rect_with_border(rec, &mut buffer, WIDTH, 0xffffff, 1);
            }
        } else {
            if window.is_key_pressed(Key::Space, KeyRepeat::No) {

                println!("{} {} \n", rec_vec[step].name, MAX_STEP);
                Rec2::draw_rect_with_border(&rec_vec[step], &mut buffer, WIDTH, 0x21FC0D, 5);
                if step == MAX_STEP {
                    step = MAX_STEP;
                } else {
                    step +=1;

                }
            }
            if window.is_key_pressed(Key::C, KeyRepeat::No) {
                println!("{}  {}\n", rec_vec[step].name, MAX_STEP);

                Rec2::draw_rect_with_border(&rec_vec[step], &mut buffer, WIDTH, 0xffffff, 5);
                if step == MIN_STEP {
                    step = 0;
                } else {
                    step -=1;
                }
            }
        } 
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}


fn treemap_recursive(
    dir_info: &DirInfo,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    result: &mut Vec<Rec2>,
    ) {
    if dir_info.size > 0 {
        let color = generate_random_hex_color(); 
        let rec = Rec2 {
            x,
            y,
            width,
            height,
            color,
            name: dir_info.name.clone(),
        };
        result.push(rec);
    }

    let total_size: u64 = dir_info.children.iter().map(|child| child.size as u64).sum();
    let children_len = dir_info.children.len();
    if children_len % 2 == 0 {
        let mut current_x = x;

        for child in &dir_info.children {
            if child.size == 0 {
                continue; // Skip zero-size directories
            }
            let child_width = ((x + width - current_x) as f64 * (child.size as f64 / total_size as f64)) as usize;
            treemap_recursive(child, current_x, y, child_width, height, result);
            current_x += child_width;
        }
    } else {
        let mut current_y = y;

        for child in &dir_info.children {
            if child.size == 0 {
                continue; // Skip zero-size directories
            }
            let child_height = ((y + height - current_y) as f64 * (child.size as f64 / total_size as f64)) as usize;
            treemap_recursive(child, x, current_y, width, child_height, result);
            current_y += child_height;
        }
    }
}


use minifb::{Key, Window, WindowOptions, KeyRepeat};
use std::cmp::min;
use crate::DirInfo;
use std::time::{SystemTime, Instant};
use crate::random;

const WIDTH: usize = 650;
const HEIGHT: usize = 650;


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
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
    name : String
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
    //    let h = Vector2 { x: WIDTH / 2, y: 0, color: 0xFF0000 }; // vert start
    //    let h1 = Vector2 { x: WIDTH / 2 , y: HEIGHT, color: 0xFF0000  }; // vert end
    //                                                                     //
    //    let v = Vector2 { x: 0, y:  HEIGHT / 2, color: 0xFF0000  }; // hor start
    //    let v1 = Vector2 { x: WIDTH , y: HEIGHT / 2, color: 0xFF0000  }; // hor end
    //
    let mut rec_vec = Vec::new();
    let mut rec1 = Rec2{
        x: 25,
        y: 25,
        width: 600,
        height: 600,
        color: 0x4A412A,
        name: String::from("Root"),
    };

    prepare_recs(&dir_info, &rec1, &mut rec_vec, true);
    println!("{rec_vec:?}");
    println!("{}", rec_vec.len());
    let mut step = 0;
    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        //    buffer.iter_mut().for_each(|pixel| *pixel = 0);
        //    for rec in &rec_vec {
        Rec2::draw_rect_with_border(&rec_vec[step], &mut buffer, WIDTH, 0xffffff, 1);
        //    }

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            if step == rec_vec.len() -1 {
                step = rec_vec.len() -2;
            }
            Rec2::draw_rect_with_border(&rec_vec[step], &mut buffer, WIDTH, 0xffffff, 1);
            step +=1;


        }
        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            Rec2::draw_rect_with_border(&rec_vec[step], &mut buffer, WIDTH, 0xffffff, 1);
            if step == 0 {
                step = 0;
            } else {

                step -=1;
            }


        }



        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

    fn prepare_recs(dir_info: &DirInfo, parent_rec: &Rec2, recs: &mut Vec<Rec2>, start: bool) {
        // let mut current_rec = parent_rec.clone();
        // recs.push(current_rec.clone());

        recs.push(parent_rec.clone());
        let mut flag = !start;

        for child in &dir_info.children {
           let mut modified_rec = parent_rec.clone();
            if start {
                modified_rec.height = ((modified_rec.height as f64 * ( child.percent_of_total / 100.0 )) as usize);  
                if modified_rec.height == 0 {
                    modified_rec.height = 20
                }
                modified_rec.color = generate_random_hex_color();

            } 
            if !start {
                modified_rec.width = ((modified_rec.width as f64 * ( child.percent_of_total / 100.0 )) as usize);
                if modified_rec.width == 0 {
                    modified_rec.width = 20
                }
                modified_rec.color = generate_random_hex_color();

            }
 
            prepare_recs(child, &modified_rec, recs, flag);
        }
    }





fn generate_random_hex_color() -> u32 {
    let seed = Instant::now().elapsed().subsec_nanos() as u32;
    let mut rng = Lcg::new(seed);

    rng.next()
}


struct Lcg {
    state: u32,
}

impl Lcg {
    fn new(seed: u32) -> Self {
        Lcg { state: seed }
    }

    fn next(&mut self) -> u32 {
        const A: u32 = 1664525;
        const C: u32 = 1013904223;

        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }
}


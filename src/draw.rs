use minifb::{Key, Window, WindowOptions};
use std::cmp::min;
use crate::DirInfo;
const WIDTH: usize = 640;
const HEIGHT: usize = 480;


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

        let mut err = dx - dy; // Initialize the error term appropriately

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
    let mut rec_vec = Vec::new();
    let mut rec1 = Rec2{
        x: 10,
        y: 10,
        width: 300,
        height: 300,
        color: 0x808080
    };
    prepare_recs(&dir_info, &rec1, &mut rec_vec);
println!("{rec_vec:?}");

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.iter_mut().for_each(|pixel| *pixel = 0);
        for rec in &rec_vec {
            Rec2::draw_rect(rec, &mut buffer, WIDTH);
        }
        //from_flat_draw_map(&mut buffer, WIDTH, &dir_info, &mut rec1);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

//Black	0x000000
//Blue 	0x0000FF
//Gray 	0x808080
//Green	0x008000
//Purple  0x800080
//White	0xFFFFFF
//86
//64
//white is root 

fn prepare_recs(dir_info: &DirInfo, parent_rec: &Rec2, recs: &mut Vec<Rec2>) {
    let mut current_rec = parent_rec.clone();
    recs.push(current_rec.clone());

    for child in &dir_info.children {
        // Update properties for the child rectangle
        current_rec.x += 5;
        current_rec.y += 5;
        current_rec.width += 10;
        current_rec.color = 0x0000FF;

        prepare_recs(child, &current_rec, recs);
    }
}

// root x,y 620 460 
// 620 - 120 
// 480 - 120
//
//    draw_rect(p_x, p_y, (640 - p_x * 2) - 120, (480 - p_y * 2) , 0x800080, buffer, width); 
//let calc: f64= (620.0 * (14.285714285714285 / 100.0)); 


//let test = calc.round();
//  draw_rect(p_x, p_y, test as usize, 480 - p_y * 2, 0x800080, buffer,width);

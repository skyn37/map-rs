use std::time::Instant;


pub fn generate_random_hex_color() -> u32 {
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

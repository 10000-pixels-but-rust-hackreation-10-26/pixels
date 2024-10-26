use rand::prelude::*;

// Synced with the colors on the frontend
#[derive(Copy, Clone)]
pub enum Color {
    White = 0,
    Green = 1,
    Yellow = 2,
    Red = 3,
    Orange = 4,
    Purple = 5,
    Blue = 6,
    Teal = 7,
    Pink = 8,
    Black = 9,
}

pub struct PixelStore {
    pub data: [Color; 10_000],
}

impl PixelStore {
    pub fn new() -> PixelStore {
        let mut rng = thread_rng();
        let mut result = PixelStore {
            // This won't be purple in reality, but for now while we just want to be sure we have results, let's do purple
            data: [Color::Purple; 10_000]
        };
        for x in result.data.iter_mut() {
            *x = unsafe { std::mem::transmute (rng.gen_range(0..9) as u8) };
        }
        result
    }
}
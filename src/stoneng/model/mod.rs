pub mod spritesheet;
pub mod tilemap;

#[allow(dead_code)]
pub struct Rect {
    top_left: (f32, f32),
    bottom_right: (f32, f32),
}
impl Rect {
    pub fn new(top_left: (f32, f32), bottom_right: (f32, f32)) -> Self {
        Self { top_left, bottom_right } 
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        // Decompose the rectangles into components
        let (a_left, a_top) = self.top_left;
        let (a_right, a_bottom) = self.bottom_right;

        let (b_left, b_top) = other.top_left;
        let (b_right, b_bottom) = other.bottom_right;
        
        let x_overlapping = a_right > b_left && a_left < b_right;
        let y_overlapping = a_top > b_bottom && a_bottom < b_top;

        return x_overlapping && y_overlapping;
    }

    pub fn intersection(&self, other: &Rect) -> Option<Rect> {
        todo!()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Color<T>(pub [T; 3]);

impl<T> Color<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self([r, g, b])
    }
}

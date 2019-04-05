#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(_r:u8, _g:u8, _b:u8) -> Color
    {
        Color
        {
            r:_r,
            g:_g,
            b:_b,
        }
    }
}
use crate::coord::Coord;
use crate::drawable::{Drawable, Pixel, Dimensions};
use crate::fonts::Font;
use crate::transform::Transform;
use crate::pixelcolor::{PixelColor, BinaryColor};
use crate::style::{Style, WithStyle};
use crate::unsignedcoord::{ToSigned, UnsignedCoord};

#[derive(Clone)]
struct LVFontBuilder<'a, C : PixelColor> {
    /// Top left corner of the text
    pub pos: Coord,

    /// Text to draw
    text: &'a str,

    /// todo: add font info.

    /// Style of the font
    style: Style<C>
}

impl<'a, C: PixelColor> Font<'a, C> for LVFontBuilder<'a, C> {
    fn render_str(text: &'a str) -> Self {
        Self {
            pos: Coord::new(0,0),
            text,
            style: Style::default()
        }
    }
}

impl<'a, C: PixelColor> Transform for LVFontBuilder<'a, C> {
    fn translate(&self, by: Coord) -> Self {
        Self {
            pos: self.pos + by,
            ..self.clone()
        }
    }

    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        unimplemented!()
    }
}

impl<'a, C: PixelColor> WithStyle<C> for LVFontBuilder<'a, C> {
    fn style(self, style: Style<C>) -> Self {
        unimplemented!()
    }

    fn stroke(self, stroke: Option<C>) -> Self {
        unimplemented!()
    }

    fn stroke_width(self, width: u8) -> Self {
        unimplemented!()
    }

    fn fill(self, stroke: Option<C>) -> Self {
        unimplemented!()
    }
}

impl<'a, C: PixelColor> Dimensions for LVFontBuilder<'a, C> {
    fn top_left(&self) -> Coord {
        self.pos
    }

    fn bottom_right(&self) -> Coord {
        unimplemented!()
    }

    fn size(&self) -> UnsignedCoord {
        unimplemented!()
    }
}

/// Pixel iterator for the `LVFontBuilder` object
struct LVFontIterator<'a, C: PixelColor> {
    char_walk_x: u32,
    char_walk_y: u32,
    current_char: Option<char>,
    idx: usize,
    pos: Coord,
    text: &'a str,
    style: Style<C>
}


impl<'a, C> IntoIterator for LVFontBuilder<'a, C>
where
    C: PixelColor + From<BinaryColor>
{
    type Item = Pixel<C>;
    type IntoIter = LVFontIterator<'a, C>;

    fn into_iter(self) -> Self::IntoIter {
        unimplemented!()
    }
}

impl<'a, C> Iterator for LVFontIterator<'a, C>
where
    C: PixelColor + From<BinaryColor>
{
    type Item = Pixel<C>;

     fn next(&mut self) -> Option<Self::Item> {
         unimplemented!()
     }
}

impl<'a, C: PixelColor> Drawable for LVFontBuilder<'a, C> {}


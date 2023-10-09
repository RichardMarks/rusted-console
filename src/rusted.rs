use crate::*;

pub struct Rusted {
    pub console: Console,
    background_color: u16,
    foreground_color: u16,
}

impl Rusted {
    pub fn new() -> Self {
        Self {
            console: Console::new(80, 25),
            background_color: 0,
            foreground_color: FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE,
        }
    }
    pub fn screen80x25(&mut self) {
        set_console_buffer_size(&mut self.console, Coord(80, 25));
    }

    pub fn screen80x50(&mut self) {
        set_console_buffer_size(&mut self.console, Coord(80, 50));
    }

    pub fn set_bgcolor(&mut self, color: u16) {
        let mut value: u16 = 0;
        if color & 1 == 1 {
            value |= BACKGROUND_RED;
        }
        if color & 2 == 2 {
            value |= BACKGROUND_GREEN;
        }
        if color & 4 == 4 {
            value |= BACKGROUND_BLUE;
        }
        if color & 8 == 8 {
            value |= BACKGROUND_INTENSITY;
        }
        self.background_color = value;
        set_console_attribute(
            &mut self.console,
            Attribute(self.foreground_color | self.background_color),
        );
    }

    pub fn set_fgcolor(&mut self, color: u16) {
        let mut value: u16 = 0;
        if color & 1 == 1 {
            value |= FOREGROUND_RED;
        }
        if color & 2 == 2 {
            value |= FOREGROUND_GREEN;
        }
        if color & 4 == 4 {
            value |= FOREGROUND_BLUE;
        }
        if color & 8 == 8 {
            value |= FOREGROUND_INTENSITY;
        }
        self.foreground_color = value;
        set_console_attribute(
            &mut self.console,
            Attribute(self.foreground_color | self.background_color),
        );
    }

    pub fn cls(&mut self) {
        let count: u16 = self.console.size.0 * self.console.size.1;
        fill_console_output_attribute(
            &mut self.console,
            count,
            Attribute(self.foreground_color | self.background_color),
            Coord(0, 0),
        );
        fill_console_output_character(&mut self.console, count, ' ', Coord(0, 0));
        set_console_cursor_position(&mut self.console, Coord(0, 0));
    }

    pub fn set_xy(&mut self, x: i32, y: i32) {
        set_console_cursor_position(&mut self.console, Coord(x as u16, y as u16));
    }

    pub fn outchar(&mut self, x: i32, y: i32, character: char) {
        set_console_cursor_position(&mut self.console, Coord(x as u16, y as u16));
        write_console(&mut self.console, character.to_string().as_str());
    }

    pub fn outchars(&mut self, x: i32, y: i32, text: &str) {
        set_console_cursor_position(&mut self.console, Coord(x as u16, y as u16));
        write_console(&mut self.console, text);
    }

    pub fn open_window(
        &mut self,
        rect: (i32, i32, i32, i32),
        fgc: u16,
        bgc: u16,
        with_frame: bool,
    ) -> Window {
        let (x, y, w, h) = rect;
        let mut wnd: Window = Window::new(x, y, w, h);

        {
            let x: u16 = x as u16;
            let y: u16 = y as u16;
            let w: u16 = w as u16;
            let h: u16 = h as u16;

            read_console_output(
                &self.console,
                &mut wnd.buffer,
                Coord(w, h),
                Coord(0, 0),
                Rect(x, y, x + w - 1, y + h - 1),
            );
        }

        self.set_bgcolor(bgc);
        self.set_fgcolor(fgc);

        self.draw_window_fill(x, y, w, h);

        if with_frame {
            self.draw_window_frame(x, y, w, h);
        }
        wnd
    }

    pub fn close_window(&mut self, window: &Window) {
        let x: u16 = window.data[0] as u16;
        let y: u16 = window.data[1] as u16;
        let w: u16 = window.data[2] as u16;
        let h: u16 = window.data[3] as u16;
        let r: u16 = x + w - 1;
        let b: u16 = y + h - 1;

        write_console_output(
            &mut self.console,
            &window.buffer,
            Coord(w, h),
            Coord(x, y),
            Rect(x, y, r, b),
        )
    }

    pub fn draw_button(&mut self, rect: (i32, i32, i32, i32), caption: &str, fgc: u16, bgc: u16) {
        let (x, y, w, h) = rect;
        self.set_bgcolor(bgc);
        self.set_fgcolor(fgc);
        self.draw_window_fill(x, y, w, h);
        self.draw_window_frame(x, y, w, h);
        self.outchars(x + 2, y + 1, caption);
    }

    fn draw_window_fill(&mut self, x: i32, y: i32, w: i32, h: i32) {
        // draw background fill
        for cy in 1..h - 1 {
            let cy: i32 = y + cy;
            for cx in 1..w - 1 {
                self.outchar(x + cx, cy, ' ');
            }
        }
    }

    fn draw_window_frame(&mut self, x: i32, y: i32, w: i32, h: i32) {
        // draw top and bottom edge
        for cx in 0..w {
            self.outchar(x + cx, y, BOX_HORIZONTAL_DOUBLE);
            self.outchar(x + cx, y + h - 1, BOX_HORIZONTAL_DOUBLE);
        }

        // draw left and right edge
        for cy in 0..h {
            self.outchar(x, y + cy, BOX_VERTICAL_DOUBLE);
            self.outchar(x + w - 1, y + cy, BOX_VERTICAL_DOUBLE);
        }

        // draw corners
        self.outchar(x, y, BOX_TOPLEFT_DOUBLE);
        self.outchar(x + w - 1, y, BOX_TOPRIGHT_DOUBLE);
        self.outchar(x, y + h - 1, BOX_BOTLEFT_DOUBLE);
        self.outchar(x + w - 1, y + h - 1, BOX_BOTRIGHT_DOUBLE);
    }
}

impl Default for Rusted {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for &'a Rusted {
    type Item = (Coord, char, (u16, u16));
    type IntoIter = RustedIter<'a>;
    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        RustedIter {
            size: self.console.size,
            char_info: &self.console.buffer,
            index: 0,
        }
    }
}

#[doc(hidden)]
pub struct RustedIter<'a> {
    size: Coord,
    char_info: &'a [CharInfo],
    index: usize,
}

impl<'a> Iterator for RustedIter<'a> {
    type Item = (Coord, char, (u16, u16));

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some((current, rest)) = self.char_info.split_first() {
            let column = (self.index as u16) % self.size.0;
            let row = (self.index as u16) / self.size.0;
            let character = current.0;
            let coord = Coord(column, row);
            let mut background_color = 0;
            let mut foreground_color = 0;
            if current.1 .0 & FOREGROUND_RED == FOREGROUND_RED {
                foreground_color |= FOREGROUND_RED;
            }
            if current.1 .0 & FOREGROUND_GREEN == FOREGROUND_GREEN {
                foreground_color |= FOREGROUND_GREEN;
            }
            if current.1 .0 & FOREGROUND_BLUE == FOREGROUND_BLUE {
                foreground_color |= FOREGROUND_BLUE;
            }
            if current.1 .0 & FOREGROUND_INTENSITY == FOREGROUND_INTENSITY {
                foreground_color |= FOREGROUND_INTENSITY;
            }
            if current.1 .0 & BACKGROUND_RED == BACKGROUND_RED {
                background_color |= FOREGROUND_RED;
            }
            if current.1 .0 & BACKGROUND_GREEN == BACKGROUND_GREEN {
                background_color |= FOREGROUND_GREEN;
            }
            if current.1 .0 & BACKGROUND_BLUE == BACKGROUND_BLUE {
                background_color |= FOREGROUND_BLUE;
            }
            if current.1 .0 & BACKGROUND_INTENSITY == BACKGROUND_INTENSITY {
                background_color |= FOREGROUND_INTENSITY;
            }
            let result = (coord, character, (background_color, foreground_color));
            self.index += 1;
            self.char_info = rest;
            Some(result)
        } else {
            self.index = 0;
            None
        }
    }
}

// #[test]
// fn foo() {
//     let mut con: Rusted = Rusted::default();
//     set_console_buffer_size(&mut con.console, Coord(14, 3));
//     con.set_bgcolor(2|4);
//     con.outchars(0, 0, "Hello, World!");
//     for (coord, character, color) in &con {
//         let Coord(column, row) = coord;
//         let (bgc, fgc) = color;
//         println!("column: {:?} row: {:?} fgc: {:?} bgc: {:?} character: {:?}", column, row, fgc, bgc, character);
//     }
//     println!("--");
//     for (coord, character, color) in &con {
//         let Coord(column, row) = coord;
//         let (bgc, fgc) = color;
//         println!("column: {:?} row: {:?} fgc: {:?} bgc: {:?} character: {:?}", column, row, fgc, bgc, character);
//     }
//     assert!(true)
// }

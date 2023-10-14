use std::fmt;

use crate::constants::*;

/// composite of background and foreground color bitmasks
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Attribute(pub u16);

/// character, attribute
#[derive(Clone, Copy, Default, PartialEq)]
pub struct CharInfo(pub char, pub Attribute);

impl fmt::Debug for CharInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "{:032b}:{:016b}", self.0 as u32, self.1.0)
        write!(f, "{:04X}:{:04X}", self.0 as u16, self.1 .0)
    }
}

/// x, y
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Coord(pub u16, pub u16);

/// a rectangle described by the left, top, right, bottom
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect(pub u16, pub u16, pub u16, pub u16);

impl Rect {
    pub fn from_xywh(x: u16, y: u16, w: u16, h: u16) -> Self {
        Self(x, y, x + w, y + h)
    }

    pub fn to_xywh(&self) -> Self {
        Self(self.0, self.1, self.2 - self.0, self.3 - self.1)
    }

    /// gets the width of the rectangle
    pub fn width(&self) -> u16 {
        self.2 - self.0
    }

    /// gets the height of the rectangle
    pub fn height(&self) -> u16 {
        self.3 - self.1
    }
}

#[derive(Debug, Clone)]
pub struct Console {
    pub size: Coord,
    pub buffer: Vec<CharInfo>,
    pub cursor: Coord,
    pub attribute: Attribute,
}

impl Console {
    pub fn new(columns: u16, rows: u16) -> Self {
        Self {
            size: Coord(columns, rows),
            buffer: vec![CharInfo::default(); (columns * rows) as usize],
            cursor: Coord(0, 0),
            attribute: Attribute(FOREGROUND_RED | FOREGROUND_GREEN | FOREGROUND_BLUE),
        }
    }
}

#[cfg(test)]
mod console {
    use crate::*;
    #[test]
    fn constructs_console() {
        let console: Console = Console::new(40, 25);
        assert_eq!(console.size.0, 40);
        assert_eq!(console.size.1, 25);
        assert_eq!(console.buffer.len(), 40 * 25);
    }
}

pub fn set_console_buffer_size(console: &mut Console, size: Coord) {
    console.buffer = vec![CharInfo::default(); (size.0 * size.1) as usize];
    console.size = size;
}

#[cfg(test)]
mod set_console_buffer_size {
    use crate::*;
    #[test]
    fn sets_console_size() {
        let mut console: Console = Console::new(10, 10);

        set_console_buffer_size(&mut console, Coord(40, 25));

        assert_eq!(console.size.0, 40);
        assert_eq!(console.size.1, 25);
        assert_eq!(console.buffer.len(), 40 * 25);
    }
}

pub fn set_console_attribute(console: &mut Console, attribute: Attribute) {
    console.attribute = attribute;
}

pub fn fill_console_output_attribute(
    console: &mut Console,
    count: u16,
    attribute: Attribute,
    start: Coord,
) {
    for n in 0..=count {
        let index: usize =
            (n as usize) + (start.0 as usize) + ((start.1 as usize) * (console.size.0 as usize));
        if index >= console.buffer.len() {
            break;
        }
        let cell: &mut CharInfo = &mut console.buffer[index];
        cell.1 = attribute;
    }
}

pub fn fill_console_output_character(
    console: &mut Console,
    count: u16,
    character: char,
    start: Coord,
) {
    for n in 0..=count {
        let index: usize =
            (n as usize) + (start.0 as usize) + ((start.1 as usize) * (console.size.0 as usize));
        if index >= console.buffer.len() {
            break;
        }
        let cell: &mut CharInfo = &mut console.buffer[index];
        cell.0 = character;
    }
}

pub fn set_console_cursor_position(console: &mut Console, position: Coord) {
    console.cursor = position;
}

pub fn write_console(console: &mut Console, text: &str) {
    for character in text.chars() {
        let index: usize =
            (console.cursor.0 as usize) + ((console.cursor.1 as usize) * (console.size.0 as usize));
        if index >= console.buffer.len() {
            break;
        }
        let cell: &mut CharInfo = &mut console.buffer[index];
        match character {
            '\n' => {
                console.cursor.0 = 0;
                console.cursor.1 += 1;
                console.cursor.1 = console.cursor.1.min(console.size.1 - 1);
            }
            ch => {
                cell.0 = ch;
                cell.1 = console.attribute;
                console.cursor.0 += 1;
                if console.cursor.0 > console.size.0 - 1 {
                    console.cursor.0 = 0;
                    console.cursor.1 += 1;
                    console.cursor.1 = console.cursor.1.min(console.size.1 - 1);
                }
            }
        }
    }
}

pub fn read_console_output(
    console: &Console,
    buffer: &mut Vec<CharInfo>,
    buffer_size: Coord,
    dst: Coord,
    src: Rect,
) {
    // println!("read_console_output {:?} {:?}", src, dst);
    let source_buffer = (&console.buffer, console.size.0, console.size.1);
    let target_buffer = (buffer, buffer_size.0, buffer_size.1);
    util_copy_buffer::<CharInfo>(source_buffer, target_buffer, src.to_xywh(), dst);
}

pub fn write_console_output(
    console: &mut Console,
    buffer: &Vec<CharInfo>,
    buffer_size: Coord,
    dst: Coord,
    src: Rect,
) {
    // println!("write_console_output {:?} {:?}", src, dst);
    let source_buffer = (buffer, buffer_size.0, buffer_size.1);
    let target_buffer = (&mut console.buffer, console.size.0, console.size.1);
    util_copy_buffer::<CharInfo>(source_buffer, target_buffer, src.to_xywh(), dst);
}

fn util_copy_buffer<T: Clone + std::fmt::Debug>(
    source_buffer: (&Vec<T>, u16, u16),
    target_buffer: (&mut Vec<T>, u16, u16),
    src: Rect,
    dst: Coord,
) {
    // println!("util_copy_buffer({:?}, {:?})", src, dst);

    let (source_vec, source_width, source_height) = source_buffer;
    let (target_vec, target_width, _target_height) = target_buffer;

    let copy_width = src.2.min(source_width);
    let copy_height = src.3.min(source_height);
    let copy_left = src.0;
    let copy_top = src.1;

    // println!("source_width: {:?}\nsource_height: {:?}\ntarget_width: {:?}\ntarget_height: {:?}", source_width, source_height, target_width, target_height);
    // println!("copy_width: {:?}\ncopy_height: {:?}\ncopy_left: {:?}\ncopy_top: {:?}", copy_width, copy_height, copy_left, copy_top);

    for copy_row in copy_top..(copy_top + copy_height) {
        let paste_row = dst.1 + (copy_row - copy_top);
        for copy_col in copy_left..(copy_left + copy_width) {
            let paste_col = dst.0 + (copy_col - copy_left);
            let copy_index = (copy_col + (copy_row * source_width)) as usize;
            if copy_index >= source_vec.len() {
                // println!("@{:?},{:?} copy index out of bounds: {:?}/{:?}", copy_col, copy_row, copy_index, source_vec.len());
                break;
            }
            let paste_index = (paste_col + (paste_row * target_width)) as usize;
            if paste_index >= target_vec.len() {
                // println!("@{:?},{:?} paste index out of bounds: {:?}/{:?}", paste_col, paste_row, paste_index, target_vec.len());
                break;
            }
            target_vec[paste_index] = source_vec[copy_index].clone();
            // println!(
            //     "copy {:?} from {:?}, {:?} @{:?} to {:?}, {:?} @{:?}",
            //     source_vec[copy_index].clone(),
            //     copy_col, copy_row, copy_index, paste_col, paste_row, paste_index
            // );
        }
    }
}

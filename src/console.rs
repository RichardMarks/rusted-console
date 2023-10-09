use crate::constants::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Attribute(pub u16);

#[derive(Debug, Clone, Copy, Default)]
pub struct CharInfo(pub char, pub Attribute);

#[derive(Debug, Clone, Copy, Default)]
pub struct Coord(pub u16, pub u16);

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect(pub u16, pub u16, pub u16, pub u16);

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
    let src_width: u16 = src.2 - src.0;
    let src_height: u16 = src.3 - src.1;
    for read_row in 0..src_height {
        let dst_row: u16 = dst.1 + read_row;
        let read_row: u16 = read_row + src.1;
        for read_col in 0..src_width {
            let dst_col: u16 = dst.0 + read_col;
            let read_col: u16 = read_col + src.0;
            let read_index: usize =
                (read_col as usize) + ((read_row as usize) * (console.size.0 as usize));
            let dst_index: usize =
                (dst_col as usize) + ((dst_row as usize) * (buffer_size.0 as usize));
            if read_index >= console.buffer.len() {
                break;
            }
            if dst_index >= buffer.len() {
                break;
            }
            let src_cell: &CharInfo = &console.buffer[read_index];
            let dst_cell: &mut CharInfo = &mut buffer[dst_index];
            dst_cell.0 = src_cell.0;
            dst_cell.1 = src_cell.1;
        }
    }
}

pub fn write_console_output(
    console: &mut Console,
    buffer: &Vec<CharInfo>,
    buffer_size: Coord,
    dst: Coord,
    src: Rect,
) {
    let src_width: u16 = src.2 - src.0;
    let src_height: u16 = src.3 - src.1;
    for read_row in 0..src_height {
        let dst_row: u16 = dst.1 + read_row;
        let read_row: u16 = read_row + src.1;
        for read_col in 0..src_width {
            let dst_col: u16 = dst.0 + read_col;
            let read_col: u16 = read_col + src.0;
            let read_index: usize =
                (read_col as usize) + ((read_row as usize) * (buffer_size.0 as usize));
            let dst_index: usize =
                (dst_col as usize) + ((dst_row as usize) * (console.size.0 as usize));
            if read_index >= buffer.len() {
                break;
            }
            if dst_index >= console.buffer.len() {
                break;
            }
            let src_cell: &CharInfo = &buffer[read_index];
            let dst_cell: &mut CharInfo = &mut console.buffer[dst_index];
            dst_cell.0 = src_cell.0;
            dst_cell.1 = src_cell.1;
        }
    }
}

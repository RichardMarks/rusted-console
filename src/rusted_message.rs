use crate::*;

pub struct RustedMessage {
    is_open: bool,
    window: Option<Window>,
    center_align_text: bool,
}

impl RustedMessage {
    pub fn new(center_align_text: bool) -> Self {
        Self {
            is_open: false,
            window: None,
            center_align_text,
        }
    }

    pub fn show(&mut self, ctx: &mut Rusted, lines: Vec<&str>) {
        if self.is_open {
            return;
        }
        let (console_width, console_height) =
            (ctx.console.size.0 as i32, ctx.console.size.1 as i32);

        let (box_width, box_height) = (
            ((console_width as f32) * 0.8) as i32,
            (lines.len() + 4) as i32,
        );

        let (box_x, box_y) = (
            (console_width - box_width) / 2,
            (console_height - box_height) / 2,
        );

        let fgc: u16 = 1 | 2 | 4 | 8;
        let bgc: u16 = 4;

        self.window = Some(ctx.open_window((box_x, box_y, box_width, box_height), fgc, bgc, true));

        if self.center_align_text {
            for (index, &line) in lines.iter().enumerate() {
                let y: i32 = (index as i32) + 2 + box_y;
                let x: i32 = (box_width - line.len() as i32) / 2;
                ctx.outchars(x, y, line);
            }
        } else {
            for (index, &line) in lines.iter().enumerate() {
                let y: i32 = (index as i32) + 2 + box_y;
                ctx.outchars(box_x + 2, y, line);
            }
        }

        self.is_open = true;
    }

    pub fn hide(&mut self, ctx: &mut Rusted) {
        if self.is_open {
            if let Some(window) = &self.window {
                ctx.close_window(window);
            }
        }
    }
}

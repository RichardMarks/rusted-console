use crate::*;

pub struct RustedChoice {
    pub selected_choice: Option<u8>,
    is_open: bool,
    window: Option<Window>,
    cursor: u8,

    question: String,
    options: Vec<String>,
    q_x: i32,
    q_y: i32,
    opt_x: i32,
    opt_y: i32,
}

impl RustedChoice {
    pub fn new() -> Self {
        Self {
            selected_choice: None,
            is_open: false,
            window: None,
            cursor: 0,
            question: String::new(),
            options: vec![],
            q_x: 0,
            q_y: 0,
            opt_x: 0,
            opt_y: 0,
        }
    }

    pub fn move_cursor_prev(&mut self) {
        if self.is_open {
            self.cursor = (self.cursor - 1) % self.options.len() as u8;
            self.selected_choice = Some(self.cursor);
        }
    }

    pub fn move_cursor_next(&mut self) {
        if self.is_open {
            self.cursor = (self.cursor + 1) % self.options.len() as u8;
            self.selected_choice = Some(self.cursor);
        }
    }

    pub fn show_yes_no(&mut self, ctx: &mut Rusted, question: &str) {
        self.show_choice(ctx, question, vec!["  Yes".to_string(), "  No".to_string()]);
    }

    pub fn show_choice(&mut self, ctx: &mut Rusted, question: &str, options: Vec<String>) {
        if self.is_open {
            return;
        }
        let (console_width, console_height) =
            (ctx.console.size.0 as i32, ctx.console.size.1 as i32);

        let (box_width, box_height) = (
            ((console_width as f32) * 0.8) as i32,
            (options.len() + 4) as i32,
        );

        let (box_x, box_y) = (
            (console_width - box_width) / 2,
            (console_height - box_height) / 2,
        );

        let (q_x, q_y) = (
            box_x + 1 - 1 + (box_width / 2) - ((question.len() / 2) as i32),
            box_y + 2,
        );

        let (opt_x, opt_y) = (box_x + 2 + (box_width / 2) - 5, 2);

        self.q_x = q_x;
        self.q_y = q_y;
        self.opt_x = opt_x;
        self.opt_y = opt_y;

        let fgc: u16 = 1 | 2 | 4 | 8;
        let bgc: u16 = 4;

        self.window = Some(ctx.open_window((box_x, box_y, box_width, box_height), fgc, bgc, true));

        self.question = question.to_string();
        self.options = options;
        self.cursor = 0;
        self.selected_choice = Some(0);

        self.redraw(ctx);

        self.is_open = true;
    }

    pub fn redraw(&self, ctx: &mut Rusted) {
        if self.is_open {
            ctx.outchars(self.q_x, self.q_y, self.question.as_str());
            for (index, opt) in self.options.iter().enumerate() {
                let y: i32 = (index as i32) + self.opt_y;
                ctx.outchars(self.opt_x, y, opt.as_str());
            }
            ctx.outchar(
                self.opt_x,
                self.opt_y + (self.cursor as i32),
                DOUBLE_RIGHT_ARROW,
            );
        }
    }

    pub fn hide(&mut self, ctx: &mut Rusted) {
        if self.is_open {
            if let Some(window) = &self.window {
                ctx.close_window(window);
            }
        }
    }
}

impl Default for RustedChoice {
    fn default() -> Self {
        Self::new()
    }
}

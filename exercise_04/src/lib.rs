pub struct TurtleGraphics {
    x: i32,
    y: i32,
    pen: i32,
    pen_down: bool,
}

impl TurtleGraphics {
    pub fn new() -> TurtleGraphics {
        TurtleGraphics {
            x: 0,
            y: 0,
            pen: 0,
            pen_down: false,
        }
    }

    fn select_pen(&mut self, pen: i32) {
        self.pen = pen;
        println!("Selected pen {}", pen);
    }

    fn pen_down(&mut self) {
        self.pen_down = true;
        println!("Pen is down");
    }

    fn pen_up(&mut self) {
        self.pen_down = false;
        println!("Pen is up");
    }

    fn move_pen(&mut self, direction: char, distance: i32) {
        let previous_x = self.x;
        let previous_y = self.y;

        match direction {
            'N' => self.y += distance,
            'S' => self.y -= distance,
            'E' => self.x += distance,
            'W' => self.x -= distance,
            _ => panic!("Invalid direction: {}", direction),
        };

        if self.pen_down {
            println!(
                "Line drawn from ({}, {}) to ({}, {})",
                previous_x, previous_y, self.x, self.y
            );
        }

        println!("Position updated to ({}, {})", self.x, self.y);
    }

    pub fn parse_command(&mut self, command: &String) {
        let args: Vec<_> = command.split(' ').collect();
        let cmd = args[0].chars().next().unwrap();
        let numeric_arg = if args.len() > 1 {
            i32::from_str_radix(args[1], 10).unwrap()
        } else {
            0
        };
        match cmd {
            'P' => self.select_pen(numeric_arg),
            'U' => self.pen_up(),
            'D' => self.pen_down(),
            'N' | 'E' | 'S' | 'W' => self.move_pen(cmd, numeric_arg),
            _ => panic!("Invalid command: {}", cmd),
        }
    }
}

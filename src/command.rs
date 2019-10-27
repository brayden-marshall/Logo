#[derive(Debug, PartialEq)]
pub enum Command {
    // movement
    Forward,
    Backward,
    Left,
    Right,
    SetHeading,
    SetXY,
    Home,

    // pen
    PenUp,
    PenDown,
    SetPenSize,
    SetPenColor,

    // other
    HideTurtle,
    ShowTurtle,
    ClearScreen,
    Clean,
    SetScreenColor,
    Show,
    Exit,
}

impl Command {
    pub fn from_string(s: &str) -> Option<Self> {
        use Command::*;
        let command = match s {
            "forward" | "fd" => Forward,
            "backward" | "bk" => Backward,
            "left" | "lt" => Left,
            "right" | "rt" => Right,
            "setheading" | "seth" => SetHeading,
            "setxy" => SetXY,
            "home" => Home,
            "penup" | "pu" => PenUp,
            "pendown" | "pd" => PenDown,
            "setpensize" => SetPenSize,
            "setpencolor" | "setpc" => SetPenColor,
            "hideturtle" | "ht" => HideTurtle,
            "showturtle" | "st" => ShowTurtle,
            "clearscreen" | "cs" => ClearScreen,
            "clean" => Clean,
            "setscreencolor" | "setsc" => SetScreenColor,
            "show" => Show,
            "exit" => Exit,
            _ => return None,
        };
        Some(command)
    }

    pub fn arity(&self) -> usize {
        use Command::*;
        match self {
            // movement
            Forward | Backward | Left | Right => 1,
            SetHeading => 1,
            SetXY => 2,
            Home => 0,

            // pen
            PenUp | PenDown => 0,
            SetPenSize => 1,
            SetPenColor => 3,

            // other
            HideTurtle | ShowTurtle => 0,
            ClearScreen | Clean => 0,
            SetScreenColor => 3,
            Show => 1,
            Exit => 0,
        }
    }
}

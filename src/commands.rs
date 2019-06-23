use std::collections::HashMap;
use turtle::Turtle;

pub struct TurtleCommand {
    pub arity: u8,
    pub func: Box<Fn(&mut Turtle, &Vec<isize>) -> ()>,
}

fn command<F: 'static>(arity: u8, func: F) -> TurtleCommand
where F: Fn(&mut Turtle, &Vec<isize>) {
    TurtleCommand {
        arity,
        func: Box::new(func),
    }
}

pub fn get_turtle_commands() -> HashMap<String, TurtleCommand> {
    vec![
        // movement
        ("forward",        command(1, forward)),
        ("fd",             command(1, forward)),
        ("backward",       command(1, backward)),
        ("bk",             command(1, backward)),
        ("left",           command(1, left)),
        ("lt",             command(1, left)),
        ("right",          command(1, right)),
        ("rt",             command(1, right)),
        ("setheading",     command(1, set_heading)),
        ("seth",           command(1, set_heading)),
        ("setxy",          command(2, set_xy)),
        ("home",           command(0, home)),
        // pen
        ("penup",          command(0, pen_up)),
        ("pu",             command(0, pen_up)),
        ("pendown",        command(0, pen_down)),
        ("pd",             command(0, pen_down)),
        ("setpensize",     command(1, set_pen_size)),
        ("setpencolor",    command(3, set_pen_color)),
        ("setpc",          command(3, set_pen_color)),
        //other
        ("hideturtle",     command(0, hide_turtle)),
        ("ht",             command(0, hide_turtle)),
        ("showturtle",     command(0, show_turtle)),
        ("st",             command(0, show_turtle)),
        ("clearscreen",    command(0, clear_screen)),
        ("cs",             command(0, clear_screen)),
        ("clean",          command(0, clean)),
        ("setscreencolor", command(3, set_screen_color)),
        ("setsc",          command(3, set_screen_color)),
        ("show",           command(0, show)),
        ("exit",           command(0, exit)),
    ].into_iter().map(|x| (x.0.to_string(), x.1)).collect()
}

fn forward(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.forward(args[0] as f64);
}

fn backward(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.backward(args[0] as f64);
}

fn right(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.right(args[0] as f64);
}

fn left(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.left(args[0] as f64);
}

fn set_heading(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.set_heading(args[0] as f64);
}

fn set_xy(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.go_to([args[0] as f64, args[1] as f64]);
}

fn home(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.home();
}

fn pen_up(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.pen_up();
}

fn pen_down(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.pen_down();
}

fn set_pen_size(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.set_pen_size(args[0] as f64);
}

fn set_pen_color(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.set_pen_color([args[0] as f64, args[1] as f64, args[2] as f64]);
}

fn hide_turtle(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.hide();
}

fn show_turtle(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.show();
}

fn clear_screen(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.clear();
    turtle.home();
}

fn clean(turtle: &mut Turtle, _args: &Vec<isize>) {
    turtle.clear();
}

fn set_screen_color(turtle: &mut Turtle, args: &Vec<isize>) {
    turtle.drawing_mut().set_background_color(
        [args[0] as f64, args[1] as f64, args[2] as f64]
    );
}

fn show(_turtle: &mut Turtle, args: &Vec<isize>) {
    println!("{}", args[0]);
}

fn exit(_turtle: &mut Turtle, _args: &Vec<isize>) {
    std::process::exit(0);
}

use std::collections::HashMap;
use turtle::Turtle;

pub type TurtleCommand = Box<Fn(&mut Turtle, &Vec<isize>) -> ()>;

pub fn get_turtle_commands() -> HashMap<String, TurtleCommand> {
    let mut commands: HashMap<String, TurtleCommand> = HashMap::new();
    commands.insert("forward".to_string(), Box::new(forward));
    commands.insert("fd".to_string(), Box::new(forward));

    commands
}

fn forward(t: &mut Turtle, args: &Vec<isize>) {
    t.forward(args[0] as f64);
}

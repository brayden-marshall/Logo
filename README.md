# Logo

A Logo language interpreter written in Rust. This interpreter supports many basic turtle graphics commands, as well as variables, procedures, repeats, and arithmetic expressions. All turtle graphics functionality is handled by the [sunjay/Turtle](github.com/sunjay/Turtle) Rust library.

## Todo before going public

- write a getting started section for the README.md

## Getting Started

Before you can use Logo, you need to install Rust and Cargo. You can find instructions on how to do that [here](https://doc.rust-lang.org/cargo/getting-started/installation.html)

To install:
- Clone this repository onto your local workstation `git clone github.com/brayden-marshall/logo`
- You can then run the interpreter with the command `cargo build` (it will take a minute or so to install dependencies)
- At this point you should be able to run the interpreter with the command `cargo run`

If you have never heard of Logo or turtle graphics before, you can familiarize yourself by checking out [this](http://cs.brown.edu/courses/bridge/1997/Resources/LogoTutorial.html) tutorial.

All supported commands are listed below, with code samples.

## Supported Commands

- Turtle movement commands (take 1 argument)
    - Forward: `fd 10 | forward 10`
    - Backward: `bk -10 | backward -10`
    - Left: `lt 90 | left 90`
    - Right: `rt -270 | right -270`

- Visual commands (take no arguments)
    - PenUp: `pu | penup`
    - PenDown: `pd | pendown`
    - HideTurtle: `ht | hideturtle`
    - ShowTurtle: `st | showturtle`
    - ClearScreen: `cs | clearscreen`
    - Clean: `clean`

- Color change commands (take three arguments [0-255] as RGB)
    - SetPenColor: `setpencolor 255 0 0 setpc 123 123 123`
    - SetScreenColor: `setscreencolor 255 0 0 setsc 123 123 123`

- Misc. turtle commands
    - SetPenSize: `setpensize 20`
    - SetHeading: `setheading 0 seth 0`
    - SetXY: `setxy 60 60`
    - Home: `home`

- Show (prints value to screen): `show 10 show :variable`
- Exit (added for convenience): `exit`

- Variables:
```
make "angle_1 45
fd 10 rt :angle_1
```

- Arithmetic operations on numbers: `fd 100 + 70 bk 7 * :var - 12`

- Repeat (can be nested): `repeat 7 [ forward 100 rt 40 ]`

- Procedures (supports parameters):
```logo
to draw_circle :x :y
pu
setxy :x :y
pd
repeat 360 [
    forward 5
    rt 1
]
end

draw_circle -50 -50
```

## Things to implement 

- Comments: `; this is a comment`

## Future Implementation Ideas

- Control Flow (if, if-else). Would require implementing boolean types as well.

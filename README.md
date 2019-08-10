# Logo

A Logo language interpreter written in Rust. This interpreter supports many basic turtle graphics commands, as well as variables, procedures, repeats, and arithmetic expressions. All turtle graphics functionality is handled by the [sunjay/Turtle](github.com/sunjay/Turtle) Rust library.

## Todo before going public

- investigate issue when parsing parentheses:
    - `show 100 + ( 200` does not give unbalanced paren error, and shows `300`

- DOCUMENT ALL OF THE THINGS!!! USE THE RUSTDOC THING!!!
    - documentation is the lord and I am it's disciple
    - I bow to the documentation as others have done before me
    - my life belongs to documentation and I would have it no other way
    - praise be to the documentation

## Supported Commands

- Forward: `fd | forward`
- Backward: `bk | backward`
- Left: `lt | left`
- Right: `rt | right`
- Show (prints value to screen): `show 10 show :variable`

- PenUp: `pu | penup`
- PenDown: `pd | pendown`
- HideTurtle: `ht | hideturtle`
- ShowTurtle: `st | showturtle`
- Home: `home`
- ClearScreen: `cs | clearscreen`
- Clean: `clean`

- SetPenSize: `setpensize 20`
- SetPenColor: `setpencolor 255 0 0 setpc 123 123 123`
- SetScreenColor: `setscreencolor 255 0 0 setsc 123 123 123`
- SetHeading: `setheading 0 seth 0`
- SetXY: `setxy 60 60`

- Repeat (can be nested): `repeat 7 [ forward 100 rt 40 ]`

- Variables:
```
make "angle_1 45
fd 10 rt :angle_1
```

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

- Arithmetic operations on numbers: `fd 100 + 70 bk 7 * :var - 12`

- Exit (added for convenience): `exit`

## Things to implement 

- Comments: `; this is a comment`

## Future Implementation Ideas

- Control Flow (if, if-else). Would require implementing boolean types as well.

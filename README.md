# Logo

A Logo language interpreter written in Rust. This interpreter supports many basic turtle graphics commands, as well as variables, procedures, repeats, and arithmetic expressions. All turtle graphics functionality is handled by the [sunjay/Turtle](github.com/sunjay/Turtle) Rust library.

## Todo before going public

- polish up error-reporting (a lot):
    - [DONE] create a new branch for error changes
    - [DONE] add runtime error types
    - refactor lexer and parser errors into error.rs
    - add proper messages for all error-types
    - location of statements in file (????) (would need a lot of changes to the lexer including row/column tracking and changing the way that newlines are handled)
    - manually test scenarios to see if messages make sense

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
- SetFillColor (shape filling): `setfillcolor 255 0 0 setfc 123 123 123`
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
- Fill (fills enclosed shape, fill is not currently implemented in the `turtle` library, so this may be tricky): `fill`
- Label: `label "something`
- Control Flow (if, if-else) (need to also implement boolean type)

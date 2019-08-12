# Logo

A Logo language interpreter written in Rust. This interpreter supports many basic turtle graphics commands, as well as variables, procedures, repeats, and arithmetic expressions. All turtle graphics functionality is handled by the [sunjay/Turtle](github.com/sunjay/Turtle) Rust library.

## Todo before going public

- write a getting started section for the README.md

- DOCUMENT ALL OF THE THINGS!!!
    - documentation is the lord and I am it's disciple
    - I bow to the documentation as others have done before me
    - my life belongs to documentation and I would have it no other way
    - praise be to the documentation

- use up arrow to re-use last command

## Supported Commands


- Turtle movement commands (take 1 argument)
    - Forward: `fd | forward`
    - Backward: `bk | backward`
    - Left: `lt | left`
    - Right: `rt | right`

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


## Things to implement 

- Comments: `; this is a comment`

## Future Implementation Ideas

- Control Flow (if, if-else). Would require implementing boolean types as well.

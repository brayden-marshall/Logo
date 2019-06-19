# Logo

## Issues

- error reporting is currently terrible and needs to be worked on, we need:
    - create error type for runtime errors in `main.rs`
    - possibly more error types: or adding value fields to error enum types if applicable
    - location that the error occured
    - implement Display on errory types instead of using Debug
    - as much information as possible in messages

- (do some research if this is the way it should be or not) allow for either CAPS or lowercase for commands, but not mixed i.e.
```
forward 100 // yes
FORWARD 100 // yes
FoRWard 100 // no
```

- find good way to be able to implement (parse is the main issue) prefix functions on numbers i.e. `random 100 sqrt 64`

- commands that take multiple arguments (setpencolor, etc.) should take an array as an argument? `setpencolor [255 123 123]`

- currently the only data type supported is Number (Variables can only be Numbers as well)

- arithmetic expressions do not support parentheses

## Supported Commands

- Forward: `fd | forward`
- Backward: `bk | backward`
- Left: `lt | left`
- Right: `rt | right`
- Show (prints value to screen): `show 10 show :variable`
- Repeat (can be nested): `repeat 7 [ forward 100 rt 40 ]`

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

- Variables:
```
make "angle_1 45
fd 10 rt :angle_1
```

- Arithmetic operations on numbers: `fd 100 + 70 bk 7 * :var - 12`

- Exit (added for convenience): `exit`

## Things to implement 
### Small-scale
- Fill (fill enclosed shape, fill is not currently implemented in the `turtle` library, so this may be tricky): `fill`
- Label: `label "something`
- Random (1 argument is max number): `forward random 100`

### Large-scale
- Procedures: 
```logo
to draw_circle
repeat 360 [
    forward 5
    rt 1
]
end

draw_circle
```

- Control Flow (if, if-else) (need to also implement boolean type)

# Logo

## Issues

- commands that take three RGB values as input do not have any range checking on arguments

- currently the only data type supported is Number (Variables can only be Numbers as well)

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

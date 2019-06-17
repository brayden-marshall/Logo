# Logo

## Issues

- commands that take three RGB values as input do not have any range checking on arguments

- currently the only data type supported is Number (Variables can only be Numbers as well)

## Thoughts For Improvement

Because I don't know what I'm doing, I'm just gonna brainstorm some things that I think I should be doing.

Here's how we're gonna deal with parsing arithmetic expressions. We're gonna use `Peekable` to always look ahead
one token and make sure that it's what we expect the next token to be. If it's a number we expect an operator and
vice versa. If we get what we expected we go on as usual with the algorithm. If we do not get what we expected
we return a parsing error. This actually shouldn't be very difficult, you got this bro. Also if it's too difficult
to evaluate the arithmetic expression as reverse polish notation, we can convert it into a tree and it'll be easily
peasily my dude. Goodnight.

## Supported Commands

- Forward: `fd | forward`
- Backward: `bk | backward`
- Left: `lt | left`
- Right: `rt | right`
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
- SetXY: `setxy 60 60`

- Variables:
```
make "angle_1 45
fd 10 rt :angle_1
```

- Exit (added for convenience): `exit`

## Things to implement 
### Small-scale
- Fill (fill enclosed shape, fill is not currently implemented in the `turtle` library, so this may be tricky): `fill`
- Label: `label <string literal>`
- Random (1 argument is max number): `forward random 100`

### Large-scale
- Arithmetic operations on numbers: `fd 100 + 70 bk sqrt 100`

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

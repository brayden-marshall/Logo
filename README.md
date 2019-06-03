# Logo

## Issues

- commands that take three RGB values as input do not have any range checking on arguments

- LexError type is currently incomplete; add more error variants to cover specific situations

- `parser.rs` is returning String as an error type; implement a ParseError type with required variants

## Thoughts For Improvement

Because I don't know what I'm doing, I'm just gonna brainstorm some things that I think I should be doing.

- remove all type-checking from the initial build of the AST, do a second pass for type-checking... this ,,should'' allow for more easily adding a symbol table later on
- refactor parser.rs to mirror lexer.rs: create a parser object, that is an iterator over expressions, etc. (not sure about this, expect further humming and hawing)

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

- Exit (added for convenience): `exit`

## Things to implement 
#### Small-scale
- Fill (fill enclosed shape, fill is not currently implemented in the `turtle` library, so this may be tricky): `fill`
- Label: `label <string literal>`
- Random (1 argument is max number): `forward random 100`

#### Large-scale
- Arithmetic operations on numbers: `fd 100 + 70 bk sqrt 100`
- Variables: 
```
make "angle 45
fd 10 rt :angle
```

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
- Control Flow (if, if-else)

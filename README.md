# Logo

## Thoughts For Improvement

Because I don't know what I'm doing, I'm just gonna brainstorm some things that I think I should be doing.

- remove all type-checking from the initial build of the AST, do a second pass for type-checking... this ,,should'' allow for more easily adding a symbol table later on

- refactor parser.rs to mirror lexer.rs: create a parser object, that is an iterator over expressions, etc.

## Supported Commands

- Forward: `fd | forward`
- Backward: `bk | backward`
- Left: `lt | left`
- Right: `rt | right`
- SetXY: `setxy 60 60`
- PenUp: `pu | penup`
- PenDown: `pd | pendown`
- HideTurtle: `ht | hideturtle`
- ShowTurtle: `st | showturtle`
- Home: `home`
- ClearScreen: `cs | clearscreen`
- Exit (added for convenience): `exit`

## Things to implement 
#### Small-scale
- Repeat: `repeat 7 [ forward 100 rt 40 ]`
- SetPenColor: `setpencolor [255 0 0] setpc [123 123 123]`
- SetFloodColor (shape filling): `setfloodcolor [255 0 0] setfc [123 123 123]`
- SetScreenColor: `setscreencolor [255 0 0] setsc [123 123 123]`
- SetPenSize: `setpensize [20 20]`
- Label: `label "string literal"`
- Random (1 argument is max number): `forward random 100`
- allow for reading input from a file as well as interactive shell (read incrementally rather than the whole file into memory at once)

#### Large-scale
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
- Arithmetic operations on numbers: `fd 100.0 + 70.0 bk sqrt 100`
- Control Flow


## Issues

- `run()` function in `main.rs` is horrible, and does not properly deal with different function aritys

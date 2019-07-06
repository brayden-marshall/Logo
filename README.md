# Logo

## Todo before going public

- add support for parameterized procedures
- polish up error-reporting (a lot):
    - create a new branch for error changes
    - add runtime error types
    - add proper messages for all error-types
    - refactor error types and related code into separate file
    - location of statements in file (????) (would need a lot of changes to the lexer including row/column tracking and changing the way that newlines are handled)
    - manually test scenarios to see if messages make sense
- write a repository description
    - we want to say that this was my first attempt at an interpreter and that it was an experiment
- delete simplify-lexer branch
- vet code and cleanup/improve where necessary (we wanna make it look good)

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

- Procedures (parameters not yet supported):
```logo
to draw_circle
repeat 360 [
    forward 5
    rt 1
]
end

draw_circle
```

- Arithmetic operations on numbers: `fd 100 + 70 bk 7 * :var - 12`

- Exit (added for convenience): `exit`

## Things to implement 

- Comments: `; this is a comment`
- Fill (fills enclosed shape, fill is not currently implemented in the `turtle` library, so this may be tricky): `fill`
- Label: `label "something`
- Control Flow (if, if-else) (need to also implement boolean type)

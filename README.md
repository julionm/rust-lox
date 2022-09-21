# RLOX

The Rlox are my rust implementation of Lox language on "Crafting Interpreters" book

### TODO

- [x] Separate my current code on different modules to be easier to work with
- Advance more on the book reading
  - I'm aiming to get to the parser part today (21/09) or tomorrow (22/09)
- Study more about error handling to guarantee that the user knows what is wrong
  - This is in progress, I've done a pretty good advance using Results and my custom errors
- Create more tests while the Lexer is small to guarantee I'll have a working language at the end of the book
- Add support for block comments

### PROBLEMS

- I've found a bug on comments, they're being considered as EOF tokens.... they should be completely ignored
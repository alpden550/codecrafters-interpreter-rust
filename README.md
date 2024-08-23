# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
Lox is an imperative, dynamically typed scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Evaluate implementation

```bash
./your_program.sh file.lox
```

Your lox file contains:

```file.lox
1 - (2 * 3) < 4 == false
2 * (3 / -"muffin")
(119 * 23 - (14 -5))
"string" + " " + "string"
```

Output:

```bash
false
[line 2] Not a number for MINUS operation.
2728
string string
```
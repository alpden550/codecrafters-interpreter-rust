# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Tokenize

Your lox file contains:

```file.lox
1 - (2 * 3) < 4 == false
2 * (3 / -"muffin")
```

```bash
sh./your_program.sh tokenize file.lox
```

```bash
./your_program.sh parse file.lox
```

Output:

```
(== (< (- 1.0 (group (* 2.0 3.0))) 4.0) false)
(* 2.0 (group (/ 3.0 (- muffin))))
```

```bash
./your_program.sh evaluate file.lox
```

Output:

```
false
Operand must be a number.
[line 2]

```
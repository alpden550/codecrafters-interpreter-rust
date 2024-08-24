# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
Lox is an imperative, dynamically typed scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## State and statements implementation

```bash
./your_program.sh file.lox
```

Your lox file contains:

```file.lox
var a = (1234 * (1456/ 44 + (1987 - 264)) / 34);
var b = true;
b = false;
print a;
print b != true;
print a == 63735.77005347593;
```

Output:

```bash
63735.77005347593
true
true
```
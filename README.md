# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
Lox is an imperative, dynamically typed scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Tokenize

Your lox file contains:

```file.lox
1 - (2 * 3) < 4 == false
2 * (3 / -"muffin")
```

```bash
./your_program.sh tokenize file.lox
```

## Parse

```bash
./your_program.sh parse file.lox
```

Output:

```
(== (< (- 1.0 (group (* 2.0 3.0))) 4.0) false)
(* 2.0 (group (/ 3.0 (- muffin))))
```

## Evaluate

```bash
./your_program.sh evaluate file.lox
```

Output:

```
false
Operand must be a number.
[line 2]
```

## Execute

Your lox file contains:

```file.lox
print "hello, Lox!";
var new = (119 * 23 - (14 -5));
var two = "string";
var three = 3 + 1 * 1000;
var bool = 3 == 3;
```

```bash
 ./your_program.sh execute file.lox
```

and output:

```bash
hello, Lox!
two=string
new=2728
bool=true
three=1003
```
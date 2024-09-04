# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
Lox is an imperative, dynamically typed scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## Implemented:

- Scanning file and parsing into tokens;
- Parsing and evaluate expressions;
- Statements and state, Global and local scopes;
- Control Flow: Conditional Execution (if statement), Logical Operators(and, or), While Loops, For Loop
- Functions: native functions(clock() as example), lox functions

```bash
./your_program.sh file.lox
```

Your lox file contains:

```file.lox
var start = clock();

fun procedure() {
  print "don't return anything";
}

var result = procedure();
print result; //  nil

fun count(n) {
  var i = 1;
  while (true) {
    if (i == n + 1) {
        return i;
    }
    print i;
    i = i + 1;
  }
}

var n = count(5);
print n;  // 6
print clock() - start;
```

Output:

```bash
don't return anything
nil
1
2
3
4
5
6
0.00002002716064453125
```
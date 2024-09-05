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
- Functions: native functions(clock() as example), lox functions, Local Functions and Closures

```bash
./your_program.sh file.lox
```

Your lox file contains:

```file.lox
fun procedure() {
  print "don't return anything";
}

var result = procedure();
print result; //  nil

fun count(n) {
  var i = 1;
  while (i <= n) {
    print i;
    i = i + 1;
  }
  return i;
}

var n = count(5);
print n;  // 6

fun fib(n) {
  if (n <= 1) {
    return n;
  }
  return fib(n - 2) + fib(n - 1);
}

var start = clock();
for (var i = 0; i < 20; i = i + 1) {
  print fib(i);
}
print clock() - start;

fun makeCounter() {
  var i = 0;
  fun count() {
    i = i + 1;
    print i;
  }

  return count;
}

var counter = makeCounter();
counter(); // "1".
counter(); // "2".
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
0
1
1
2
3
5
8
13
21
34
55
89
144
233
377
610
987
1597
2584
4181
0.02478313446044922
1
2
```
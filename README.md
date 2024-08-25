# RLox

Rust interpreter for
[Lox](https://craftinginterpreters.com/the-lox-language.html), a simple
Lox is an imperative, dynamically typed scripting language.

This code follows the book
[Crafting Interpreters](https://craftinginterpreters.com/) by Robert Nystrom.

## State, statements and global and local scopes implementation

```bash
./your_program.sh file.lox
```

Your lox file contains:

```file.lox
var a = "global a";
var b = "global b";
var c = "global c";
{
    var a = "outer a";
    var b = "outer b";
        {
            var a = "inner a";
            print a;
            print b;
            print c;
        }
    print a;
    print b;
    print c;
}
print a;
print b;
print c;

{
    var calculated = (1234 * (1456/ 44 + (1987 - 264)) / 34);
    var is_true = true == !nil;
    print calculated;
    print is_true;
    print (true == !"") == (!false);
}
print calculated;
print is_true;
```

Output:

```bash
inner a
global b
global c
outer a
outer b
global c
global a
global b
global c
63735.77005347593
true
true
[line 28] Not founded value in the scopes for variable calculated
[line 29] Not founded value in the scopes for variable is_true
```
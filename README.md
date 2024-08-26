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
- Control Flow: Conditional Execution (if statement), Logical Operators(and, or)

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

if (!nil) {print "Not None";} else {print "None";}
if (nil) {print "Not None";} else {print "None";}

print "hi" or 2; // "hi".
print nil or "yes"; // "yes"
print 1 and 0; // 0
print 1 and 1; // 1
```

Output:

```bash
inner a
outer b
global c
inner a
outer b
global c
inner a
outer b
global c
63735.77005347593
true
true
Not None
None
hi
yes
0
1
[line 28] Not founded value in the scopes for variable calculated
[line 29] Not founded value in the scopes for variable is_true
```
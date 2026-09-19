# PLANNING

## Expressions vs Statements
An expression is surrounded by parenthesis, while a statement is the body of an expression terminated by a semicolon. For example:

| Goal | Statement | Expression |
| ---- | --------- | ---------- |
| Add 1 + 2 | `1+2;` | `(1+2)` |

## Evaluation
Statements are immediately evaluated. Expressions aren't evaluated until called, at which time they become a statement. Expressions are evaluated with `#`. In the addition example given above, `(1+2)`, this would be evaluated like so: `#(1+2);`. This now has both converted it into a statement and evaluated it.

## Evaluation vs Construction
Evaluation, done with `#`, is left-associative. For example, `f # g # h` in mathematical representation is `h(g(f()))`. Meanwhile, `@`, used for construction, is right-associative, meaning `f @ g @ h` in mathematical representation is `f(g(h()))`. What is important to note is that the two of these create different things. `#` is a statement. `@` creates an expression.

## Important Things About Construction
### 1. Construction is not a way to pass arguments into functions

If you have a single argument function `f` that takes in an integer `a`, `f @ a` is not passing `a` into `f`. It is binding `a` to `f`'s first parameter. This is still an expression, just one with no unbound parameters. Assume you do `let value = (f @ a);`. Every time you want to get "value", you have to do `#value` and "call the function" (evaluate the expression) to get the actual result. What would be the correct thing to do is to do something like `let value = a # f;`. Now value is the result of the expression instead of a new expression

### 2. Currying and construction
Suppose you have a function `f` that takes three parameters, `x`, `y`, and `z`, and you want to cover the first and third in a construction. There are a few ways to do this:

**Named method**

`f <- (x=1, z=2)`

**Positional method**

`f <- (1, _, 2)`

## Branching
Branching can be done via a pattern match or an if structure. Match statements look like this:
```
(value){|
    a => r1,
    b => r2,
|}
```

Where r1 and r2 are different results. Of course, this is an expression, so we have to put our input and a hashtag before the expression to do anything. It could look something like this:
```
let not = (input){|
    0 => 1,
    1 => 0,
|}

let not_1 = 1 # not;
```

If statements look like this:
```
cond ? true : false
```

Else-if chains look like this:
```
cond_1 ? result_1 :? cond_2 ? result_2 : result_3

// or like this

cond_1 ? result_1
:? cond_2 ? result_2
: result_3
```

## Parameters
Generally, you name your parameters in your type declaration for an expression, like so:
```
let greet:(name:string)->() = (print @ fmt "hello, %s" name);
```

However, if you don't declare your type and instead leave it to the compiler, ~in punishment~ your parameters will be named a to z in order, then a1-z1 when you have more than 26. For example:
```
let greet = (print @ fmt "hello, %s" a);
```

Parameters can be made generic like so:
```
let show:(thing:T~Showable)->() = (print thing);

// Or, more generically (ha-ha):
let expr:(name:type~trait)->() = (...);
```
Where trait is something that `type` must implement. To not require a trait, use `All`.

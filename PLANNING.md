# PLANNING

## Variables and Types
Gearbox has its own type inference system, but sometimes you will need to specify the type yourself when Gearbox can't figure it out.
```gearbox
let inferenced = 12;
let explicit:i32 = 12;
```

Here are the types Gearbox has:
| Name | Gearbox Type |
| ---- | ------------ |
| Signed 32-bit integer | `int32` |
| Unsigned 32-bit integer | `uint32` |
| Signed 16-bit integer | `int16` |
| Unsigned 16-ibt integer | `uint16` |
| Signed 8-bit integer | `int8` |
| Unsigned 8-bit integer | `uint8` |
| Signed 64-bit integer | `int64` |
| Unsigned 64-bit integer | `uint64` |
| Byte | `byte` |
| Boolean | `bool` |
| Signed 8 bit char | `char8` |
| Unsigned 8 bit char | `uchar8` |
| Signed 16 bit char | `char16` |
| Unsigned 16 bit char | `uchar16` |
| 32-bit float | `float32` |
| 64-bit float | `float64` |
| Function | `(argtype, argtype)[capturetype, capturetype] -> returntype` |
| Unit | `()` |

Each type also has a default that can be referenced via a different name. This is syntactic sugar, so the interpreter and compiler will still see `int32` or `char8`

| Default Name | Type |
| ------------ | ---- |
| `int` | `int32` |
| `char` | `char8` |
| `float` | `float32` |

## Functions
Functions are done entirely via lambda-style functions. A simple function can be done like so:
```gearbox
let add = (a, b)[] => a + b;

let subtract = (a, b)[] => {return a - b};

// The absolute bare minimum:
let nullproc = ()[] => 0;
```

More complex function bodies have to be surrounded with curly braces (`{}`) in order to permit multiple statements.

The square brackets are for variable capturing. In Gearbox, functions are intended to be entirely closed, meaning that any outside data a function wants to access has to be passed in via parameters. This could be a struggle for closures though, so captures have been added to functions to allow them to take in variables from the outside scope without having to have them passed in as arguments. For example:
```gearbox
let constant = 12;

let add_constant = (n)[constant] => n+constant;
```

In this way, constant is now passed in without having to be an argument, allowing for functions to do something like this:
```gearbox
let make_adder = (amt) => (n)[amt] => n+1;

let add_two = make_adder 2; // or make_adder(2);
add_two 4;                  // returns 6
```

## Function Types and Type Parameters
You can either declare types in the variable storing the function or in the function itself. Hell, even both if you want. If you define it in the function, it will be passed to the variable holding it.
```gearbox
let in_func = (a:int, b:int)[]->int => a + b;

let in_bind:(int, int)->int = (a, b)[] a + b;

let in_both:(int, int)->int = (a:int, b:int)[]->int => a + b;

let implicit = (a, b)[] => a + b;
```

Some parts of function typing are not required, while others are. For example, return can be implicit like so:
```gearbox
let implicit_return = (a:int, b:int)[] = a + b;
```


Type parameters are also enabled, though the syntax might be different than what you're used to.
``` gearbox
let print = T:trait ~ (val:T) => print val;
```

## Conditionals and Pattern Matching
If statements are relatively simple.
```gearbox
// Simple if statement
cond ? true : false

// Else-if chain
cond1 ? result1 : cond2 ? result2 : cond3 ? result3 : result4

// Better written else-if chain
cond1 ? result1 : 
    cond2 ? result2 : 
    cond3 ? result3 : 
    result4;
```

There are also match statements, similar to Rust, but Gearbox's are more similar to functions.
```
let result = |val| => {
    val1 -> result1,
    val2 -> result2,
    val3 -> result3,
}
```

## Loop blocks
Gearbox has only a basic loop, similar to Rust's `loop` blocks.
```gearbox
loop loopName => {
    // code
    break loopName;
}
```

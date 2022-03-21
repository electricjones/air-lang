# Air Language
This is a weekend attempt at creating an interpreter from scratch for a made-up scripting language. No Virtual Machine,
no true compiler, and no optimizations. Just a simple interpreter to parse and execute simple math expressions.

**Please don't use for anything you care about**

## Air Language Script
Called "air" because it sounds a little like the first part of "arithmetic". Also, I will use any excuse to sneak in
some *Last Airbender* reference 🥲

The language itself is just a series of arithmetic expressions that get evaluated down to a single `i32` integer. The
purpose was not to create something useful, but to understand parsing and execution.

The script
```
1 + 1 - (2 + 1 + (3 - 2)) + 12
```
Evaluates to `10`. The expression is evaluated from the inner most parentheses outward.

Right now, only `i32` and `+` and `-` are supported.

## Usage
At the moment, there is no CLI. Just a hardcoded `main()` function.

## Roadmap
With the rest of my (long) weekend, I hope to add:

- A CLI that allows you to run a `.air` file or directly input an expression
- Add `*`, `/`, `^x`, `++`, and `--`
- A useful Readme

If I get done with that, or ever revist the project, I would

- Allow for sets of number `[1, 2, 3]` with operators `+[1, 2, 3] = 6`
- Allow for labels (like variables) and multiple lines `a = 1 + 2 / 3; b = 4^2; b - a` would return the last statement

That's really it. My next project is to build a rudimentary Virtual Machine and actual Turing Complete toy language, so
that's where I would get into scope, function, and all the rest.

## Sources

- https://pest.rs/
- https://doc.rust-lang.org/book/
- https://createlang.rs/

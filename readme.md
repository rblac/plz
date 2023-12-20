# plzerror
A toy [PL/0](https://en.wikipedia.org/wiki/PL/0) parser, made while loosely following the first Lox interpreter tutorial in [Crafting Interpreters](https://craftinginterpreters.com).
Written by me(Roch Błachut) as the grading assignment for my 2023 Programming Languages class.

The [`pl0.ebnf`](./pl0.ebnf) file contains an EBNF description of PL/0's grammar, which I ripped straight from Wikipedia and subsequently made some minor tweaks to.
From what I've seen, nearly every implementation of PL/0 differs from others in some minor way, be it comment syntax or operator selection. This makes sense, considering its primary role is that of an educational exercise, and I've elected not to break this trend for the sake of some "canonical correctness".

In brief, in this version of PL/0:
- comments start with a `#` symbol and terminate at the end of a line
- `==` and `!=` are the equality and inequality operators, respectively
- `?` prints the value of the identifier placed immediately after it

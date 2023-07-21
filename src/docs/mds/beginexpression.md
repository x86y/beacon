# Left Parenthesis (`(`)
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#parentheses)

## `( ...`: Begin Expression

Starts an expression, and only one expression. Must end with a corresponding `)`.

`(` supercedes any precedence order, so that an expression in `()` is evaluated fully before it can be used in the outer context.

```bqn
   1 + 2 - 3
0

   (1 + 2) - 3
0
```

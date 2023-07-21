# Leftwards Arrow With Hook (`↩`)

## `n ↩ v`: Change
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#assignment)

Changes the value of variable with name `n` to value `v`.

Variable `n` must already exist.

```bqn
   a ↩ 1

   ⊢ b ← 3
3

   ⊢ b ↩ "Be the change you wish to see in the world."
"Be the change you wish to see in the world."

```
## `n F↩`: Modify
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#assignment)

Apply function `F` to existing variable `n`, and assign the result back to `n`.

```bqn
   ⊢ b ⌽↩

```
## `n F↩ v`: Modify
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#assignment)

Assign `n F v` to `n`.

```bqn
   ⊢ b ↓˜↩ 6
```

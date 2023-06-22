# Logical And (`∧`)

## `∧ 𝕩`: Sort Up
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#sort)

Sort array `𝕩` in ascending order.

```bqn
   ∧ 3‿1‿4‿1‿5
⟨ 1 1 3 4 5 ⟩



```
## `𝕨 ∧ 𝕩`: Logical And
[→full documentation](https://mlochbaum.github.io/BQN/doc/logic.html)

Logical And of `𝕨` and `𝕩`.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 ∧ 1
1

   1‿0 ∧ 1‿1
⟨ 1 0 ⟩
```

# Logical Or (`∨`)

## `∨ 𝕩`: Sort Down
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#sort)

Sort array `𝕩` in descending order.

```bqn
   ∨ 3‿1‿4‿1‿5
⟨ 5 4 3 1 1 ⟩



```
## `𝕨 ∨ 𝕩`: Logical Or
[→full documentation](https://mlochbaum.github.io/BQN/doc/logic.html)

Logical Or of `𝕨` and `𝕩`.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 ∨ 0
1

   1‿0 ∨ 1‿1
⟨ 1 1 ⟩

   0 ∨ 0
0
```

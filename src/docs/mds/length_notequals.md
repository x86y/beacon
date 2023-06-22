# Not Equal (`≠`)

## `≠ 𝕩`: Length
[→full documentation](https://mlochbaum.github.io/BQN/doc/shape.html)

Length of the first dimension of `𝕩`.


```bqn
   ≠ 3
1

   ≠ ⟨1, 2, 3⟩
3

   ≠ 3‿4‿5⥊0
3

   ≠ 1‿4‿5⥊0
1

   ≠ 4‿4‿5⥊0
4



```
## `𝕨 ≠ 𝕩`: Not Equal To
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#comparisons)

Do argument atoms not match?

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 ≠ 3
1

   2‿3‿0 ≠ 3‿1‿0
⟨ 1 1 0 ⟩

   'a' ≠ 'a'
0
```

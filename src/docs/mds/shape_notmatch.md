# Not Identical To (`≢`)

## `≢ 𝕩`: Shape
[→full documentation](https://mlochbaum.github.io/BQN/doc/shape.html)

Length of each dimension of x.

```bqn
   ≢ 1
⟨⟩

   ≢ 1‿2
⟨ 2 ⟩

   ≢ 1‿2 ≍ 3‿4
⟨ 2 2 ⟩



```
## `𝕨 ≢ 𝕩`: Not Match
[→full documentation](https://mlochbaum.github.io/BQN/doc/match.html)

Does `𝕨` not exactly match `𝕩`?

```bqn
   1 ≢ ⟨1⟩
1

   ⟨1⟩ ≢ ⟨1⟩
0
```

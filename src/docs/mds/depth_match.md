# Identical To (`≡`)

## `≡ 𝕩`: Depth
[→full documentation](https://mlochbaum.github.io/BQN/doc/depth.html)

Highest level of nesting in `𝕩`.

```bqn
   ≡ 2‿3‿4
1

   ≡ ⟨2,<3,4,<<<5⟩
4

   ≡ 9
0



```
## `𝕨 ≡ 𝕩`: Match
[→full documentation](https://mlochbaum.github.io/BQN/doc/match.html)

Does `𝕨` exactly match `𝕩`?

```bqn
   1 ≡ ⟨1⟩
0

   ⟨1⟩ ≡ ⟨1⟩
1
```

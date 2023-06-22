# Greater Than (`>`)

## `> 𝕩`: Merge
[→full documentation](https://mlochbaum.github.io/BQN/doc/couple.html)

Combine an array of arrays into one array. All elements of `𝕩` must have the same rank, and the result rank is that plus the rank of `𝕩`.

Returns and boxed atoms unchanged.


```bqn
   a ← ⟨⟨1, 2⟩, ⟨3, 4⟩⟩

   >a
┌─     
╵ 1 2  
  3 4  
      ┘

   ≢a
⟨ 2 ⟩

   ≢>a
⟨ 2 2 ⟩




```
## `𝕨 > 𝕩`: Greater Than
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#comparisons)

`𝕨` and `𝕩` can both be either numbers or characters.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 > 3
0

   2‿3‿0 > 3‿1‿0
⟨ 0 1 0 ⟩

   'a' > 'b'
0
```

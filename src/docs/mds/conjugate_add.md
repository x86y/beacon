# Plus (`+`)

## `+ 𝕩`: Conjugate
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#basic-arithmetic)

Complex conjugate of `𝕩`. BQN doesn't support complex numbers yet, so it has no effect.

```bqn
   + 1
1

   + ¯1
¯1


```
## `𝕨 + 𝕩`: Add
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#basic-arithmetic)

`𝕨` added to `𝕩`. Either `𝕨` or `𝕩` can be a character, and if so, the other has to be an integer.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 + 2
3

   1 + 2‿3‿4
⟨ 3 4 5 ⟩

   'a' + 4
'e'
```

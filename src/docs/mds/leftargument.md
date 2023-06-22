# Mathematical Double-struck W (`𝕨`)

## `𝕨`: Left Argument
[→full documentation](https://mlochbaum.github.io/BQN/doc/block.html#arguments)

A variable assigned to the left argument of a block. `𝕎` can be used to access the left argument as a function.

```bqn
   5 {𝕨} 1
5

   -‿÷ {𝕎𝕩}¨ 4
⟨ ¯4 0.25 ⟩

```
In a call with no left argument, `𝕨` functions as [Nothing](nothing.md) and `𝕎` can't be used.

```bqn
   {(-𝕨)⋈𝕩} 6
⟨ 6 ⟩

   2 {(-𝕨)⋈𝕩} 6
⟨ ¯2 6 ⟩
```

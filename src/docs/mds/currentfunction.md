# Mathematical Double-struck S (`𝕊`)

## `𝕊`: Current Function
[→full documentation](https://mlochbaum.github.io/BQN/doc/block.html#self-reference)

A variable assigned to the current function block. `𝕤` accesses the same value but has a subject role.

`𝕊` can be used for recursion.

```bqn
   F ← {𝕊 0: 1; 𝕩 × 𝕊 𝕩-1} 
   F 5
120

   {𝕤‿𝕤}4
⟨ (function block) (function block) ⟩
```

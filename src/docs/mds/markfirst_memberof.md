# Element Of (`∊`)

## `∊ 𝕩`: Mark Firsts
[→full documentation](https://mlochbaum.github.io/BQN/doc/selfcmp.html#mark-firsts)

Mark the first occurrence of each major cell in `𝕩` with a 1, and all other occurrences with a 0.

```bqn
   ∊ 4‿5‿6‿6‿4‿7‿5
⟨ 1 1 1 0 0 1 0 ⟩

   a ← 3‿3 ⥊ ↕9

   ∊ a
⟨ 1 1 1 ⟩



```
## `𝕨 ∊ 𝕩`: Member Of
[→full documentation](https://mlochbaum.github.io/BQN/doc/search.html#member-of)

Is each cell in `𝕨` a major cell of `𝕩`?

```bqn
   ⟨1⟩ ∊ ↕9
⟨ 1 ⟩

   b ← 3‿3 ⥊ ↕9

   ⟨0‿1‿2⟩ ∊ b
┌·   
· 0  
    ┘

   ⟨1‿3 ⥊ 0‿1‿2⟩ ∊ b
┌·   
· 0  
    ┘
```

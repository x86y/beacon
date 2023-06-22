# Circle Backslash (`⍉`)

## `⍉ 𝕩`: Transpose
[→full documentation](https://mlochbaum.github.io/BQN/doc/transpose.html)

Move the first axis of `𝕩` to the end.

```bqn
   a ← 3‿3 ⥊ ↕9

   ⍉ a
┌─       
╵ 0 3 6  
  1 4 7  
  2 5 8  
        ┘

   b ← 1‿2‿3 ⥊ ↕6

   ≢⍉ b
⟨ 2 3 1 ⟩



```
## `𝕨 ⍉ 𝕩`: Reorder Axes
[→full documentation](https://mlochbaum.github.io/BQN/doc/transpose.html)

Rearrange the axes of `𝕩` as per the axis indices in `𝕨`.

```bqn
   ≢ c ← 2‿3‿4‿5‿6 ⥊1

   ≢ 1‿3‿2‿0‿4 ⍉ c
⟨ 5 2 4 3 6 ⟩
```

# Epsilon Underbar (`⍷`)

## `⍷ 𝕩`: Deduplicate
[→full documentation](https://mlochbaum.github.io/BQN/doc/selfcmp.html#deduplicate)

Unique major cells of `𝕩`.

```bqn
   ⍷ 4‿5‿6‿6‿4‿7‿5
⟨ 4 5 6 7 ⟩

   a ← 3‿3 ⥊ ↕6

   ⍷ a
┌─       
╵ 0 1 2  
  3 4 5  
        ┘



```
## `𝕨 ⍷ 𝕩`: Find
[→full documentation](https://mlochbaum.github.io/BQN/doc/find.html)

Mark the top left location of the occurrences of `𝕨` in `𝕩` with a 1, and other locations with 0.

Result is the same shape as `(≢𝕨)↕x`.

```bqn
   "string" ⍷ "substring"
⟨ 0 0 0 1 ⟩

   "loooooong" ⍷ "short"
⟨⟩

   b ← 7 (4|⋆˜)⌜○↕ 9

   c ← (0‿3‿0≍0‿1‿0)

   c ⍷ b
┌─               
╵ 0 0 0 0 0 0 0  
  0 0 0 0 0 0 0  
  0 0 0 0 0 0 0  
  0 0 1 0 0 0 1  
  0 0 0 0 0 0 0  
  0 0 1 0 0 0 1  
                ┘
```

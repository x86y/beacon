# Up Down Arrow (`↕`)

## `↕ 𝕩`: Range
[→full documentation](https://mlochbaum.github.io/BQN/doc/range.html)

Return all indices that would index into an array of shape `𝕩`.

When given a single number, range from `0` to `𝕩-1`.

```bqn
   ↕ 4
⟨ 0 1 2 3 ⟩

   ↕ 4‿5
┌─                                         
╵ ⟨ 0 0 ⟩ ⟨ 0 1 ⟩ ⟨ 0 2 ⟩ ⟨ 0 3 ⟩ ⟨ 0 4 ⟩  
  ⟨ 1 0 ⟩ ⟨ 1 1 ⟩ ⟨ 1 2 ⟩ ⟨ 1 3 ⟩ ⟨ 1 4 ⟩  
  ⟨ 2 0 ⟩ ⟨ 2 1 ⟩ ⟨ 2 2 ⟩ ⟨ 2 3 ⟩ ⟨ 2 4 ⟩  
  ⟨ 3 0 ⟩ ⟨ 3 1 ⟩ ⟨ 3 2 ⟩ ⟨ 3 3 ⟩ ⟨ 3 4 ⟩  
                                          ┘



```
## `𝕨 ↕ 𝕩`: Windows
[→full documentation](https://mlochbaum.github.io/BQN/doc/windows.html)

Overlapping slices from `𝕩` of shape `𝕨`.

```bqn
   5 ↕ "abcdefg"
┌─       
╵"abcde  
  bcdef  
  cdefg" 
        ┘

   a ← 3‿3⥊↕9

   2‿2 ↕ a
┌─     
┆ 0 1  
  3 4  
       
  1 2  
  4 5  
       
       
  3 4  
  6 7  
       
  4 5  
  7 8  
      ┘
```

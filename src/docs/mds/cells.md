# Breve (`˘`)

## `𝔽˘ 𝕩`, `𝕨 𝔽˘ 𝕩`: Cells
[→full documentation](https://mlochbaum.github.io/BQN/doc/rank.html#cells)

Apply `𝔽` to/between the major cells of the arguments. (`𝔽⎉¯1`)

```bqn
   a ← 3‿3 ⥊ ↕9


   <˘ a
⟨ ⟨ 0 1 2 ⟩ ⟨ 3 4 5 ⟩ ⟨ 6 7 8 ⟩ ⟩

   a ≍˘ a
┌─       
╎ 0 1 2  
  0 1 2  
         
  3 4 5  
  3 4 5  
         
  6 7 8  
  6 7 8  
        ┘
```

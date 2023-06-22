# Square Image Of (`⊏`)

## `⊏ 𝕩`: First Cell
[→full documentation](https://mlochbaum.github.io/BQN/doc/select.html#first-cell)

First major cell of `𝕩`.

```bqn
   ⊏ ⟨1, 2, 3⟩
┌·   
· 1  
    ┘

   a ← 3‿3 ⥊ ↕9

   ⊏ a
⟨ 0 1 2 ⟩



```
## `𝕨 ⊏ 𝕩`: Select
[→full documentation](https://mlochbaum.github.io/BQN/doc/select.html)

Select the major cells of `𝕨` at the indices in `𝕩`.

```bqn
   2‿0 ⊏ ⟨1, 2, 3⟩
⟨ 3 1 ⟩

   b ← 3‿3 ⥊ ↕9

   2‿0 ⊏ b
┌─       
╵ 6 7 8  
  0 1 2  
        ┘
```

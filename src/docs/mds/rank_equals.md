# Equal (`=`)

## `= 𝕩`: Rank
[→full documentation](https://mlochbaum.github.io/BQN/doc/shape.html)

Returns the number of dimensions in `𝕩`.


```bqn
   = 0
0

   = 3⥊0
1

   = 3‿3⥊0
2

   3‿3‿3 ⥊ ⟨⟨0⟩⟩
┌─                   
╎ ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
                     
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
                     
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
  ⟨ 0 ⟩ ⟨ 0 ⟩ ⟨ 0 ⟩  
                    ┘



```
## `𝕨 = 𝕩`: Equal To
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#comparisons)

Do argument atoms match?

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 = 3
0

   2‿3‿0 = 3‿1‿0
⟨ 0 0 1 ⟩

   'a' = 'a'
1
```

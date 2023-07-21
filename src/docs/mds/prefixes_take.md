# Up Arrow (`↑`)

## `↑ 𝕩`: Prefixes
[→full documentation](https://mlochbaum.github.io/BQN/doc/prefixes.html)

Prefixes of array `𝕩` along its first axis.

```bqn
   ↑ 1‿2‿3‿4
⟨ ⟨⟩ ⟨ 1 ⟩ ⟨ 1 2 ⟩ ⟨ 1 2 3 ⟩ ⟨ 1 2 3 4 ⟩ ⟩

   a ← 3‿3 ⥊ ↕9

   ↑ a
┌─                                    
· ↕0‿3 ┌─        ┌─        ┌─         
       ╵ 0 1 2   ╵ 0 1 2   ╵ 0 1 2    
               ┘   3 4 5     3 4 5    
                         ┘   6 7 8    
                                   ┘  
                                     ┘



```
## `𝕨 ↑ 𝕩`: Take
[→full documentation](https://mlochbaum.github.io/BQN/doc/take.html)

For each integer in `𝕨`, take that many elements from each dimension of `𝕩`.

Negative numbers take from the end.

If any of the elements in `𝕨` are greater than the length of their respective dimension, the dimension is extended with a fill value.

```bqn
   3 ↑ 1‿3‿5‿67
⟨ 1 3 5 ⟩

   b ← 4‿4 ⥊ ↕16

   3‿3 ↑ b
┌─        
╵ 0 1  2  
  4 5  6  
  8 9 10  
         ┘

   5‿5 ↑ b
┌─               
╵  0  1  2  3 0  
   4  5  6  7 0  
   8  9 10 11 0  
  12 13 14 15 0  
   0  0  0  0 0  
                ┘

   3‿¯3 ↑ b
┌─         
╵ 1  2  3  
  5  6  7  
  9 10 11  
          ┘
```

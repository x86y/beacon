# Down Arrow (`↓`)

## `↓ 𝕩`: Suffixes
[→full documentation](https://mlochbaum.github.io/BQN/doc/prefixes.html)

Suffixes of array `𝕩` along its first axis.

```bqn
   ↓ 1‿2‿3‿4
⟨ ⟨ 1 2 3 4 ⟩ ⟨ 2 3 4 ⟩ ⟨ 3 4 ⟩ ⟨ 4 ⟩ ⟨⟩ ⟩

   a ← 3‿3 ⥊ ↕9

   ↓ a
┌─                                    
· ┌─        ┌─        ┌─        ↕0‿3  
  ╵ 0 1 2   ╵ 3 4 5   ╵ 6 7 8         
    3 4 5     6 7 8           ┘       
    6 7 8           ┘                 
          ┘                           
                                     ┘



```
## `𝕨 ↓ 𝕩`: Drop
[→full documentation](https://mlochbaum.github.io/BQN/doc/take.html)

For each integer in `𝕨`, drop that many elements from the beginning of each dimension of `𝕩`.

Negative numbers drop from the end.

```bqn
   3 ↓ 1‿3‿5‿67
⟨ 67 ⟩

   b ← 4‿4 ⥊ ↕16

   3‿3 ↓ b
┌─    
╵ 15  
     ┘

   5‿5 ↓ b
┌┐
└┘


   3‿¯3 ↓ b
┌─    
╵ 12  
     ┘
```

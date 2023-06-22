# Tape (`≍`)

## `≍ 𝕩`: Solo
[→full documentation](https://mlochbaum.github.io/BQN/doc/couple.html)

Add a dimension to `𝕩`.

```bqn
   ≍ 1
⟨ 1 ⟩


   ≍≍ 1
┌─   
╵ 1  
    ┘


   ≍≍≍ 1
┌─   
╎ 1  
    ┘


   ≍≍ 1‿2‿3‿4
┌─         
╎ 1 2 3 4  
          ┘


   ≍≍≍ 1‿2‿3‿4
┌─         
┆ 1 2 3 4  
          ┘



```
## `𝕨 ≍ 𝕩`: Couple
[→full documentation](https://mlochbaum.github.io/BQN/doc/couple.html)

Join `𝕨` and `𝕩` along a newly created axis.

```bqn
   1 ≍ 3
⟨ 1 3 ⟩

   1‿2 ≍ 2‿3
┌─     
╵ 1 2  
  2 3  
      ┘
```

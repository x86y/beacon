# Barb (`⥊`)

## `⥊ 𝕩`: Deshape
[→full documentation](https://mlochbaum.github.io/BQN/doc/reshape.html)

Put all elements of `𝕩` in a rank 1 array, promoting to an array if necessary.

```bqn
   ⥊ 1
⟨ 1 ⟩

   ⥊ 1‿2 ≍ 3‿4
⟨ 1 2 3 4 ⟩



```
## `𝕨 ⥊ 𝕩`: Reshape
[→full documentation](https://mlochbaum.github.io/BQN/doc/reshape.html)

Put all elements of `𝕩` in an array of shape `𝕨`, removing elements or repeating them cyclically if necessary.

A single element in `𝕩` can be a function, which will be replaced with an appropriate length:
- `∘` Exact fit
- `⌊` Round length down, discarding elements
- `⌽` Round length up
- `↑` Round length up, and use element fill to add extra elements.

```bqn
   3‿3 ⥊ 3
┌─       
╵ 3 3 3  
  3 3 3  
  3 3 3  
        ┘

   2‿⌽‿2 ⥊ 1‿2‿3
┌─     
╎ 1 2  
       
  3 1  
      ┘

   2‿↑‿2 ⥊ 1‿2‿3
┌─     
╎ 1 2  
       
  3 0  
      ┘
```

# Circle Stile (`⌽`)

## `⌽ 𝕩`: Reverse
[→full documentation](https://mlochbaum.github.io/BQN/doc/reverse.html)

Reverse `𝕩` along the first axis.

```bqn
   ⌽ 1‿2‿3
⟨ 3 2 1 ⟩

   a ← 3‿3 ⥊ ↕9

   ⌽ a
┌─       
╵ 6 7 8  
  3 4 5  
  0 1 2  
        ┘



```
## `𝕨 ⌽ 𝕩`: Rotate
[→full documentation](https://mlochbaum.github.io/BQN/doc/reverse.html#rotate)

Move the first `𝕨` elements of `𝕩` to its end. Negative `𝕨` reverses the direction of rotation.

```bqn
   2 ⌽ 1‿2‿3
⟨ 3 1 2 ⟩

   b ← 3‿3 ⥊ ↕9

   2 ⌽ b
┌─       
╵ 6 7 8  
  0 1 2  
  3 4 5  
        ┘
```

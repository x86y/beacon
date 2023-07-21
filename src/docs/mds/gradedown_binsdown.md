# Del Stile (`⍒`)

## `⍒ 𝕩`: Grade Down
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#grade)

Indices of `𝕩` that would sort its major cells in descending order.

```bqn
   a ← 1‿2‿3

   ⍒ a
⟨ 2 1 0 ⟩

   (⍒a) ⊏ a
⟨ 3 2 1 ⟩



```
## `𝕨 ⍒ 𝕩`: Bins Down
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#bins)

Binary search for each cell of `𝕩` in `𝕨`, returning the number of major cells in `𝕨` greater than or equal to that cell.

`𝕨` must be sorted in descending order.

[Right Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   7‿5‿4‿3 ⍒ 2
┌·   
· 4  
    ┘

   7‿5‿4‿3 ⍒ 2‿6
⟨ 4 1 ⟩
```

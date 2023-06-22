# Delta Stile (`⍋`)

## `⍋ 𝕩`: Grade Up
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#grade)

Indices of `𝕩` that would sort its major cells in ascending order.

```bqn
   a ← 3‿2‿1

   ⍋ a
⟨ 2 1 0 ⟩

   (⍋a) ⊏ a
⟨ 1 2 3 ⟩




```
## `𝕨 ⍋ 𝕩`: Bins Up
[→full documentation](https://mlochbaum.github.io/BQN/doc/order.html#bins)

Binary search for each cell of `𝕩` in `𝕨`, returning the number of major cells in `𝕨` less than or equal to that cell.

`𝕨` must be sorted in ascending order.

```bqn
   3‿4‿5‿7 ⍋ 2
┌·   
· 0  
    ┘

   3‿4‿5‿7 ⍋ 2‿6
⟨ 0 3 ⟩
```

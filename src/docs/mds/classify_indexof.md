# Square Original Of (`⊐`)

## `⊐ 𝕩`: Classify
[→full documentation](https://mlochbaum.github.io/BQN/doc/selfcmp.html#classify)

Translate major cells of `𝕩` to unique ID numbers based on first occurrence.

```bqn
   ⊐ 5‿6‿2‿2‿5‿1
⟨ 0 1 2 2 0 3 ⟩

   a ← 3‿3 ⥊ 0‿1‿2‿9‿0‿9‿0‿1‿2

   ⊐ a
⟨ 0 1 0 ⟩



```
## `𝕨 ⊐ 𝕩`: Index Of
[→full documentation](https://mlochbaum.github.io/BQN/doc/search.html#index-of)

First index of each major cell of `𝕩` in `𝕨`. Rank of `𝕩` must be at least cell rank of `𝕨`.

If a cell is not found in `𝕨`, the length of `𝕨` (`≠𝕨`) is used for that position.

```bqn
   5‿6‿2‿2‿5‿1 ⊐ 5‿7‿1‿6
⟨ 0 6 5 1 ⟩

   b ← 3‿3 ⥊ 0‿1‿2‿9‿0‿9‿0‿1‿2

   b ⊐ ≍9‿0‿9
⟨ 1 ⟩
```

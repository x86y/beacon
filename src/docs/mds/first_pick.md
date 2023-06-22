# Square Image Of Or Equal To (`⊑`)

## `⊑ 𝕩`: First
[→full documentation](https://mlochbaum.github.io/BQN/doc/pick.html#first)

First element of `𝕩`.

```bqn
   ⊑ ⟨1, 2, 3⟩
1

   a ← 3‿3 ⥊ ↕9

   ⊑ a
0



```
## `𝕨 ⊑ 𝕩`: Pick
[→full documentation](https://mlochbaum.github.io/BQN/doc/pick.html)

Pick the element of `𝕩` at index `𝕨`.

```bqn
   2 ⊑ ⟨1, 2, 3⟩
3

   b ← 3‿3 ⥊ ↕9

   2‿0 ⊑ b
6
```

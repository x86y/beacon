# Square Original Of or Equal To (`⊒`)

## `⊒ 𝕩`: Occurrence Count
[→full documentation](https://mlochbaum.github.io/BQN/doc/selfcmp.html#occurrence-count)

Number of times each major cell of `𝕩` has occurred before the current position.

```bqn
   ⊒   2‿7‿1‿8‿1‿7‿1‿8‿2‿8‿4
⟨ 0 0 0 0 1 1 2 1 1 2 0 ⟩

   ≍⟜⊒ 2‿7‿1‿8‿1‿7‿1‿8‿2‿8‿4
┌─                       
╵ 2 7 1 8 1 7 1 8 2 8 4  
  0 0 0 0 1 1 2 1 1 2 0  
                        ┘



```
## `𝕨 ⊒ 𝕩`: Progressive Index Of
[→full documentation](https://mlochbaum.github.io/BQN/doc/search.html#progressive-index-of)

Index of the first unused match of each major cell of `𝕩` in `𝕨`. If there are no more matches left, the length of `𝕨` is placed in that position.

```bqn
   "aaa" ⊒ "aaaaa"
⟨ 0 1 2 3 3 ⟩

   "aaabb" ⊒ "ababababab"
⟨ 0 3 1 4 2 5 5 5 5 5 ⟩
```

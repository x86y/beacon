# Lazy S (`∾`)

## `∾ 𝕩`: Join
[→full documentation](https://mlochbaum.github.io/BQN/doc/join.html)

Join all elements of `𝕩` together.

Element ranks must be compatible.

```bqn
   ∾ ⟨1‿2, 3, 4‿5⟩
⟨ 1 2 3 4 5 ⟩

   m ← (3‿1≍⌜4‿2‿5) ⥊¨ 2‿3⥊↕6

   ∾ m
┌─                       
╵ 0 0 0 0 1 1 2 2 2 2 2  
  0 0 0 0 1 1 2 2 2 2 2  
  0 0 0 0 1 1 2 2 2 2 2  
  3 3 3 3 4 4 5 5 5 5 5  
                        ┘



```
## `𝕨 ∾ 𝕩`: Join To
[→full documentation](https://mlochbaum.github.io/BQN/doc/join.html)

Join `𝕨` to `𝕩` along the first axis.

```bqn
   "abcd" ∾ "EFG"
"abcdEFG"

   a ← 3‿3 ⥊ ↕9

   c ← 4‿3 ⥊ ↕12

   a∾c
┌─         
╵ 0  1  2  
  3  4  5  
  6  7  8  
  0  1  2  
  3  4  5  
  6  7  8  
  9 10 11  
          ┘
```

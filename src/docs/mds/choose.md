# Circle with Lower Right Quadrant (`◶`)

## `𝔽◶𝕘 𝕩`, `𝕨 𝔽◶𝕘 𝕩`: Choose
[→full documentation](https://mlochbaum.github.io/BQN/doc/choose.html)

Apply `𝔽` to the arguments and use the result to [pick](first_pick.md#𝕨--𝕩-pick) (`⊑`) a function from list `𝕘`. Apply the picked function to the arguments.

```bqn
   F ← ⊢◶+‿-‿÷‿×

   F 0
0

   F 1
¯1

   F 2
0.5

   F 3
1
```

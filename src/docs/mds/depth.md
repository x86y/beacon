# Circle With Two Dots (`⚇`)

## `𝔽⚇𝕘 𝕩`, `𝕨 𝔽⚇𝕘 𝕩`: Depth
[→full documentation](https://mlochbaum.github.io/BQN/doc/depth.html#the-depth-modifier)

Apply `𝔽` to the cells of the arguments at depth given in `𝕘`. Negative numbers count down from the top level and non-negative ones from the bottom up.


```bqn
   1⊸↓⚇1 ⟨⟨1,2,3⟩, ⟨4,5,6⟩⟩
⟨ ⟨ 2 3 ⟩ ⟨ 5 6 ⟩ ⟩

   1 ↓⚇1 ⟨⟨1,2,3⟩, ⟨4,5,6⟩⟩
⟨ ⟨ 2 3 ⟩ ⟨ 5 6 ⟩ ⟩

   (+´↕)⚇0 ⟨2,4‿7,3⟩  
⟨ 1 ⟨ 6 21 ⟩ 3 ⟩
```

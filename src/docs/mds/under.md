# Circle Jot (`⌾`)

## `𝔽⌾𝔾 𝕩`, `𝕨 𝔽⌾𝔾 𝕩`: Under
[→full documentation](https://mlochbaum.github.io/BQN/doc/under.html)

- Apply transformation `𝔾` to all arguments
- Apply `𝔽` to the transformed arguments
- Undo transformation `𝔾`

Where `𝔾` must be

- A function invertible by `⁼` (Undo)
- A structural modification

```bqn
   9⌾(1⊸⊑) 1‿2‿3
⟨ 1 9 3 ⟩

   √⁼ (√1) + (√9)
16

   1 +⌾√ 9
16
```

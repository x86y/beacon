# Solidus (`/`)

## `/ 𝕩`: Indices
[→full documentation](https://mlochbaum.github.io/BQN/doc/replicate.html#indices)

Repeat the index of each element in `𝕩` by the element's value. `𝕩` must be rank 1.

```bqn
   / 1‿2‿3
⟨ 0 1 1 2 2 2 ⟩

   / 1‿0‿1
⟨ 0 2 ⟩



```
## `𝕨 / 𝕩`: Replicate
[→full documentation](https://mlochbaum.github.io/BQN/doc/replicate.html)

Repeat each major cell in `𝕩` by the corresponding element in `𝕨`.

Unit `𝕨` applies to all elements.

```bqn
   3 / "copy"
"cccooopppyyy"

   1‿0‿1 / 1‿2‿3
⟨ 1 3 ⟩
```

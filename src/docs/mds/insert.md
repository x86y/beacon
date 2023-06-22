# Double Acute Accent (`˝`)

## `𝔽˝ 𝕩`: Insert
[→full documentation](https://mlochbaum.github.io/BQN/doc/fold.html)

Fold over cells of `𝕩` with `𝔽` from end to start, that is, insert `𝔽` between the major cells of `𝕩`.

```bqn
   a ← 3‿3 ⥊ ↕9

   +˝ a
⟨ 9 12 15 ⟩

   0‿1‿2 + 3‿4‿5 + 6‿7‿8
⟨ 9 12 15 ⟩


```
## `𝕨 𝔽˝ 𝕩`: Insert With Initial
[→full documentation](https://mlochbaum.github.io/BQN/doc/fold.html#initial-element)

Monadic insert, but use `𝕨` as initial right argument.

If

```bqn
   b ← 3‿3 ⥊ ↕9

   1‿1‿1 +˝ b
⟨ 10 13 16 ⟩

   1 +˝ b
⟨ 10 13 16 ⟩

   0‿1‿2 + 3‿4‿5 + 6‿7‿8 + 1‿1‿1
⟨ 10 13 16 ⟩
```

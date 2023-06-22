# Circle Star (`⍟`)

## `𝔽⍟𝔾 𝕩`, `𝕨 𝔽⍟𝔾 𝕩`: Repeat
[→full documentation](https://mlochbaum.github.io/BQN/doc/repeat.html)

Apply `𝔾` to `𝕨` and `𝕩`, then apply `𝔽` to `𝕩` that may times. If `𝕨` is given, use it each time as a constant left argument.

If `𝔾` returns an array, give `𝔽⍟𝕩` for each of its elements.

```bqn
   1 +⍟⊢ 4
8

   1 +⍟1‿2‿3 4
⟨ 5 6 7 ⟩

   3 ∾⍟{≠𝕩} ⟨4,5,6⟩
⟨ 3 3 3 4 5 6 ⟩
```

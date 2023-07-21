# Circled Triangle Down (`⎊`)

## `𝔽⎊𝔾 𝕩`, `𝕨 𝔽⎊𝔾 𝕩`: Catch
[→full documentation](https://mlochbaum.github.io/BQN/doc/assert.html#catch)

Apply `𝔽` to the arguments.

If an error happens when `𝔽` is applied, cancel its execution, apply `𝔾` to the arguments and return its result.

Otherwise, return the result of `𝔽`.

```bqn
   ∾⎊{"error occurred with argument: "∾•Fmt 𝕩} 1
"error occurred with argument: 1"

   ∾⎊{"error occurred with argument: "∾•Fmt 𝕩} ⟨⟨1,2⟩, ⟨3,4⟩⟩
⟨ 1 2 3 4 ⟩
```

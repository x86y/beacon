# Left Pointing Double Angle Quotation (`«`)

## `« 𝕩`: Nudge Back
[→full documentation](https://mlochbaum.github.io/BQN/doc/shift.html)

Remove the first element of `𝕩`,  add a cell of fill values to the end of the first axis of `𝕩`.

```bqn
   78 « 1‿2‿3
⟨ 2 3 78 ⟩

   « 1‿2‿3
⟨ 2 3 0 ⟩

   « 3‿3 ⥊ 9
┌─       
╵ 9 9 9  
  9 9 9  
  0 0 0  
        ┘



```
## `𝕨 « 𝕩`: Shift Before
[→full documentation](https://mlochbaum.github.io/BQN/doc/shift.html)

Remove the first `≠𝕨` (length) major cells from `𝕩`, join `𝕨` to the end of `𝕩`. Ranks must match.

```bqn
   8‿5 « 1‿2‿3
⟨ 3 8 5 ⟩

   a ← 3‿3 ⥊ 9

   1‿2‿3 « a
┌─       
╵ 9 9 9  
  9 9 9  
  1 2 3  
        ┘
```

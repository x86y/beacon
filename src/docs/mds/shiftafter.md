# Right Pointing Double Angle Quotation (`»`)

## `» 𝕩`: Nudge
[→full documentation](https://mlochbaum.github.io/BQN/doc/shift.html)

Remove the last element of `𝕩`, add a cell of fill values to the start of the first axis of `𝕩`.

```bqn
   » 1‿2‿3
⟨ 0 1 2 ⟩

   » 3‿3 ⥊ 9
┌─       
╵ 0 0 0  
  9 9 9  
  9 9 9  
        ┘



```
## `𝕨 » 𝕩`: Shift After
[→full documentation](https://mlochbaum.github.io/BQN/doc/shift.html)

Remove the last `≠𝕨` (length) major cells from `𝕩`, join `𝕨` to the start of `𝕩`. Ranks must match.

```bqn
   78 » 1‿2‿3
⟨ 78 1 2 ⟩

   1‿2 » 1‿2‿3
⟨ 1 2 1 ⟩

   a ← 3‿3 ⥊ 9

   1‿2‿3 » a
┌─       
╵ 1 2 3  
  9 9 9  
  9 9 9  
        ┘
```

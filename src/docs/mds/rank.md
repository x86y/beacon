# Circled Horizontal Bar With Notch (`⎉`)

## `𝔽⎉𝕘 𝕩`, `𝕨 𝔽⎉𝕘 𝕩`: Rank
[→full documentation](https://mlochbaum.github.io/BQN/doc/rank.html#rank)

Apply `𝔽` to cells at ranks given in `𝕘`. Non-negative numbers indicate the rank of the cell and negative ones indicate the difference from full rank.

The ranks applied are given by the following:

- `⎉ c`     Rank-c cells of `𝕩` (monadic) or both arguments (dyadic)
- `⎉ b‿c`   Rank-b cells of `𝕨` and rank-c cells of `𝕩` (dyadic)
- `⎉ a‿b‿c` Rank-a cells of `𝕩` (monadic), b-cells of `𝕨` and c-cells of `𝕩` (dyadic)


```bqn
   a ← 3‿2‿4⥊"ABCDEFGHIJKLMNOPQRSTUVWXYZ"

   ⌽⎉2 a
┌─      
╎"EFGH  
  ABCD  
        
 ·MNOP  
  IJKL  
        
 ·UVWX  
  QRST" 
       ┘
```

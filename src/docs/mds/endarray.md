# Right Square Bracket (`]`)

## `... ]`: End array
[→full documentation](https://mlochbaum.github.io/BQN/doc/arrayrepr.html#high-rank-arrays)

Ends an array started by a `[`. See [Begin Array](beginarray.md) for more details.

```bqn
   ["abc", "def"]
┌─     
╵"abc  
  def" 
      ┘

   [↕4, ↕5]
>: Elements didn't have equal shapes (contained shapes ⟨4⟩ and ⟨5⟩)
```

# Left Square Bracket (`[`)

## `[ ...`: Begin array
[→full documentation](https://mlochbaum.github.io/BQN/doc/arrayrepr.html#high-rank-arrays)

Starts a high-rank array. Entries must be separated by `,` or `⋄`. These must have the same shape. They become major cells of the result.

Must end with a corresponding `]`.

```bqn
   ["abc", "def"]
┌─     
╵"abc  
  def" 
      ┘

   [↕4, ↕5]
>: Elements didn't have equal shapes (contained shapes ⟨4⟩ and ⟨5⟩)
```

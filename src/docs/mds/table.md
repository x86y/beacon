# Top Left Corner (`⌜`)

## `𝕨 𝔽⌜ 𝕩`: Table
[→full documentation](https://mlochbaum.github.io/BQN/doc/map.html)

Apply `𝔽` between every possible pair of the elements of the arguments.

```bqn
   1‿2‿3‿4 +⌜ 4‿5‿6‿7
┌─           
╵ 5 6  7  8  
  6 7  8  9  
  7 8  9 10  
  8 9 10 11  
            ┘

   "abc" ∾⌜ "xyz"
┌─                
╵ "ax" "ay" "az"  
  "bx" "by" "bz"  
  "cx" "cy" "cz"  
                 ┘
```

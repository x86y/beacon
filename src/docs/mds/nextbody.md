# Semicolon (`;`)

## `;`: Next Body
[→full documentation](https://mlochbaum.github.io/BQN/doc/block.html#multiple-bodies)

End the current block body and start a new one. [Headers](header.md) (`:`) and [predicates](predicate.md) (`?`) can control which body is evaluated. A function can have two headers without these, indicating the monadic and dyadic cases.

```bqn
   3 { 𝕩÷2 ; -𝕨‿𝕩 } 4   
⟨ ¯3 ¯4 ⟩

   F ← {𝕊a‿b: a-b; 𝕊a‿b‿c: b+c}

   F 5‿2      
3

   F 1‿3‿6         
9
```

# Left Curly Bracket (`{`)

## `{ ...`: Begin Block
[→full documentation](https://mlochbaum.github.io/BQN/doc/block.html)

Starts a block, which can be one of:

- Function
- 1-Modifier
- 2-Modifier
- Namespace
- Immediate Block

Must end with a corresponding `}`.

```bqn
   {𝕨 + 𝕩}   
(function block)

   {𝕨‿𝔽‿𝕩}   
(1-modifier block)

   {𝕨‿𝔽‿𝔾‿𝕩} 
(2-modifier block)

   {a ⇐ 5}   
{a⇐}

   {5+4+6}   
15
```

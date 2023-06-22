# Colon (`:`)

## `:`: Header
[→full documentation](https://mlochbaum.github.io/BQN/doc/block.html#block-headers)

Placed at the end of a block header. A header has syntax that matches the way the block is called. It indicates the block type, and number and structure of inputs.

```bqn
   "xy" {a‿b _op c: b} ∞
'y'

```
Multiple bodies are searched in order to find one with a matching header.

```bqn
   F ← {m Fn n: m+Fn n;  𝕊n: 2×n;  𝕊⁼𝕩: 𝕩÷2}

   F 3      
6

   F⁼ 6     
3

   10 F 3   
16
```

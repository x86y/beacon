# Comma (`,`) and Diamond (`⋄`)

## `,` or `⋄`: Separator
[→full documentation](https://mlochbaum.github.io/BQN/doc/token.html#separators)

Separates statements in blocks, programs, and arrays. Characters `,` and `⋄` are interchangeable with each other and with newline.

```bqn
   a ← 3 , ⊢ b ← 2

   1 ⋄ 2 , 3
3

   ⟨1 , 2 ⋄ 3⟩
⟨ 1 2 3 ⟩

   {1 ⋄ 2 ⋄ 3}
3
```

# Times (`×`)

## `× 𝕩`: Sign
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#basic-arithmetic)

Sign of `𝕩`.
- `¯1` if `𝕩 < 0`
- `0` if `𝕩 = 0`
- `1` if `𝕩 > 0`

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   × ¯5‿0‿5‿1
⟨ ¯1 0 1 1 ⟩



```
## `𝕨 × 𝕩`: Multiply
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#basic-arithmetic)

`𝕨` multiplied by `𝕩`.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 × 2
2

   2 × 2‿3‿4
⟨ 4 6 8 ⟩
```

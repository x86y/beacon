# Acute Accent (`´`)

## `𝔽´ 𝕩`: Fold
[→full documentation](https://mlochbaum.github.io/BQN/doc/fold.html)

Fold over `𝕩` with `𝔽` from right to left i.e. Insert `𝔽` between the elements of `𝕩`.

`𝕩` must be a simple list (`1 = =𝕩`).

```bqn
   +´ 1‿2‿3
6

   1+2+3
6

   -´ 1‿2‿3
2

   1-2-3
2


```
## `𝕨 𝔽´ 𝕩`: Fold With Initial
[→full documentation](https://mlochbaum.github.io/BQN/doc/fold.html#initial-element)

Monadic fold, but use `𝕨` as initial right argument.

```bqn
   5 +´ 1‿2‿3
11

   1+2+3+5
11

   5 -´ 1‿2‿3
¯3

   1-2-3-5
¯3
```

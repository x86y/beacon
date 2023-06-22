# Grave (`` ` ``)

## ``𝔽` 𝕩``: Scan
[→full documentation](https://mlochbaum.github.io/BQN/doc/scan.html)

Scan over `𝕩` with `𝔽` from left to right, producing intermediate values.


```bqn
   +` 1‿2‿3
⟨ 1 3 6 ⟩

   ⟨1, 1+2, (1+2)+3⟩
⟨ 1 3 6 ⟩

   -` 1‿2‿3
⟨ 1 ¯1 ¯4 ⟩

   ⟨1, 1-2, (1-2)-3⟩
⟨ 1 ¯1 ¯4 ⟩


```
## ``𝕨 𝔽` 𝕩``: Scan With initial

Monadic scan, but use `𝕨` as initial left argument.

```bqn
   5 +` 1‿2‿3
⟨ 6 8 11 ⟩

   ⟨5+1, (5+1)+2, ((5+1)+2)+3⟩
⟨ 6 8 11 ⟩

   5 -` 1‿2‿3
⟨ 4 2 ¯1 ⟩

   ⟨5-1, (5-1)-2, ((5-1)-2)-3⟩
⟨ 4 2 ¯1 ⟩
```

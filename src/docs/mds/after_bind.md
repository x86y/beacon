# Left Multimap (`⟜`)

## `𝔽⟜𝕘 𝕩`: Bind
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html#bind)

Supply `𝕘` as a right argument to `𝔽` (`𝕩 𝔽 𝕘`).

`𝕘` is a constant, `𝔽` must be dyadic.

```bqn
   -⟜3 9
6

   - 3 9
Double subjects (missing ‿?)

   9 - 3
6



```
## `𝔽⟜𝔾 𝕩`: After
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html)

Apply `𝔾` to `𝕩`, and supply it as a right argument to `𝔽` (`𝕩 𝔽 (𝔾 𝕩)`).

`𝔽` must be dyadic, `𝔾` must be monadic.

```bqn
   ×⟜- 9
¯81

   × - 9
¯1

   9 × (- 9)
¯81



```
## `𝕨 𝔽⟜𝔾 𝕩`: Dyadic After
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html)

Apply `𝔾` to `𝕩`, and supply it as a right argument to `𝔽` (`𝕨 𝔽 (𝔾 𝕩)`).

`𝔽` must be dyadic, `𝔾` must be monadic.

```bqn
   2 ×⟜- 1
¯2

   2 × (- 1)
¯2
```

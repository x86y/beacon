# Multimap (`⊸`)

## `𝕗⊸𝔾 𝕩`: Bind Left
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html#bind)

Supply `𝕗` as a left argument to `𝔾` (`𝕗 𝔾 𝕩`).

`𝕗` is a constant, `𝔾` must be dyadic.

```bqn
   3⊸- 9
¯6

   3 - 9
¯6



```
## `𝔽⊸𝔾 𝕩`: Before
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html)

Apply `𝔽` to `𝕩`, and supply it as a left argument to `𝔾` (`(𝔽 𝕩) 𝔾 𝕩`).

`𝔽` must be monadic, `𝔾` must be dyadic.

```bqn
   -⊸+ 9
0

   - + 9
¯9

   (- 9) + 9
0



```
## `𝕨 𝔽⊸𝔾 𝕩`: Dyadic Before
[→full documentation](https://mlochbaum.github.io/BQN/doc/hook.html)

Apply `𝔽` to `𝕨`, and supply it as a left argument to `𝔾` (`(𝔽 𝕨) 𝔾 𝕩`).

`𝔽` must be monadic, `𝔾` must be dyadic.

```bqn
   2 -⊸+ 1
¯1

   2 - + 1
1

   (- 2) + 1
¯1
```

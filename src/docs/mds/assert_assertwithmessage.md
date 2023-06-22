# Exclamation Mark (`!`)

## `! 𝕩`: Assert
[→full documentation](https://mlochbaum.github.io/BQN/doc/assert.html#assert)

Throw an error if `𝕩` is not 1.

```bqn
   ! 1
1

   ! 2
Assertion error

   ! "hello"
hello




```
## `𝕨 ! 𝕩`: Assert With Message
[→full documentation](https://mlochbaum.github.io/BQN/doc/assert.html#assert)

Throw an error with message `𝕨` if `𝕩` is not 1.

```bqn
   "hi" ! 1
1

   "two" ! 2
two

   "hello error" ! "hello"
hello error
```

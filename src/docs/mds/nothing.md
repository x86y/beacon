# Middle Dot (`·`)

## `·`: Nothing
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#nothing)

Indicates no value. If a left argument is Nothing, the function is called with no left argument, and if the right is Nothing, it's not called and "returns" Nothing.

```bqn
   · ⌽ "abc"  
"cba"

```
### In Trains
[→full documentation](https://mlochbaum.github.io/BQN/doc/train.html#2-train-3-train)

Nothing can serve as a left argument in a train to string together multiple monadic functions.

```bqn
   (-+-) 5
¯10

   (-·+-) 5
5

```
### Destructuring
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#destructuring)

For pattern matching in assignment or a block header, Nothing indicates an unused value.

```bqn
   F ← {𝕊 a‿·‿b: a∾b}

   F 1‿2‿3
⟨ 1 3 ⟩
```

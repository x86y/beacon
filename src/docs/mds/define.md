# Leftwards Arrow (`←`)

## `n ← v`: Define
[→full documentation](https://mlochbaum.github.io/BQN/doc/expression.html#assignment)

Defines a new variable with name `n` and value `v`.

Variable `n` must not already exist in the scope.

```bqn
   ⊢ a ← 1

   ⊢ b ← 3‿3 ⥊ 5
┌─       
╵ 5 5 5  
  5 5 5  
  5 5 5  
        ┘

   C ← ↑
```

# Lesser Than (`<`)

## `< 𝕩`: Enclose
[→full documentation](https://mlochbaum.github.io/BQN/doc/enclose.html)

Create a unit array containing `𝕩`. (`(≢<𝕩) ≡ ⟨⟩`)


```bqn
   <1
┌·   
· 1  
    ┘


   ≢<1
⟨⟩



```
## `𝕨 < 𝕩`: Lesser Than
[→full documentation](https://mlochbaum.github.io/BQN/doc/arithmetic.html#comparisons)

`𝕨` and `𝕩` can both be either numbers or characters.

[Pervasive.](https://mlochbaum.github.io/BQN/doc/arithmetic.html#pervasion)

```bqn
   1 < 3
1

   2‿3‿0 < 3‿1‿0
⟨ 1 0 0 ⟩
```

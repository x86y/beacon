# Diaresis (`¨`)

## `𝔽¨ 𝕩`, `𝕨 𝔽¨ 𝕩`: Each
[→full documentation](https://mlochbaum.github.io/BQN/doc/map.html)

Apply `𝔽` to/between the elements of the arguments. (`𝔽⚇¯1`)

```bqn
   <¨ 1‿2‿3
┌─                   
· ┌·    ┌·    ┌·     
  · 1   · 2   · 3    
      ┘     ┘     ┘  
                    ┘

   4‿5‿6 ∾¨ 1‿2‿3
⟨ ⟨ 4 1 ⟩ ⟨ 5 2 ⟩ ⟨ 6 3 ⟩ ⟩
```

# Full Stop (`.`)

## `ns . name`: Namespace Field
[→full documentation](https://mlochbaum.github.io/BQN/doc/namespace.html)

Access a field with name `name` in namespace `ns`. Field must have been exported with `⇐`.

```bqn
   {a⇐1} . a
1

   {F⇐-}.F 5
¯5
```

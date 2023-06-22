# Leftward Double Arrow (`⇐`)

## `n ⇐ v`: Export Definition
[→full documentation](https://mlochbaum.github.io/BQN/doc/namespace.html#exports)

Define a variable with name `n` and export it from the current namespace.

```bqn
   ns ← { exported ⇐ 5, unexported ← 0}
   ns.exported
5
   ns.unexported
No key found

```
## `n ⇐`: Export names
[→full documentation](https://mlochbaum.github.io/BQN/doc/namespace.html#exports)

Export the names given in `n` from the current namespace. Names must be defined somewhere in the scope.

```bqn
   ns1 ← { ⟨alsoexported⟩⇐, exported ⇐ 5, alsoexported ← 0}
   ns1.exported
5
   ns1.alsoexported
0
```

<h1 align="center">Beacon</h1>
<p align="center">
A native and a cross platform guide for BQN array programming language.
</p>
<p align="center">
    <img width="400" src="https://github.com/x86y/beacon/blob/main/assets/demo.png" alt="demo"/>
</p>

## Features
- \` + KEY combination typing for BQN primitives
- Glyph toolbar (hover documentation missing for now)
- Tabs (Ctrl + n,p to go to next/prev tab & Ctrl + t,q to open/close a tab)
- History & tabs persistence
- Cross-platform


## Linux/MacOS instructions
```sh
> git clone https://github.com/x86y/beacon
> cd beacon
> cargo run --release
```

## Windows instructions
```sh
> git clone https://github.com/x86y/beacon
> cd beacon
> BQN_WASM=./BQN.wasm cargo run --release --no-default-features --features=bqnwasm
```

The code relies a lot on detegr's bqn bindings, and uses tooltip documentation files from his bqnlsp/genhelp tool! Check it out at:
https://sr.ht/~detegr/bqnlsp/

# apple-store-substrate
<div align="center">
<img src="https://user-images.githubusercontent.com/20852667/50734764-602a1900-11e7-11e9-9283-3eca3b8b5aab.png" width="150px">
</div>
An apple store on substrate

## Build and Start apple-store chain
```
./init.sh
./build.sh
cargo build --release
./target/release/apple-store --dev
```

## Interactions with the runtime
To simplify interactions with the runtime, we can use:
<https://polkadot.js.org/apps/>

To have it connect to your local node, set:
<div align="center">
<img width="100px" src="https://user-images.githubusercontent.com/20852667/50735241-25c47a00-11ef-11e9-8238-99abf1eee942.png">
</div>

Then, sending extrinsics and getting the chain state :raised_hands:
<div align="center">
<img width="1000px" src="https://user-images.githubusercontent.com/20852667/50735044-82259a80-11eb-11e9-947c-5e094feb6c69.png">
</div>

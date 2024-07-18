# cw-account-management

# Instantiate
- [x] admin
- [x] signers
- [x] threshold
- [x] whitelist enabled

# Execute
- [x] change admin
- [x] add/remove signers
- [x] change threshold
- [x] execute account tx
- [x] sign account tx by signer
- [x] set/remove whitelist addresses
- [x] set/remove transfer limit
- [x] enable/disable whitelist address

# Query
- [x] query admin
- [x] query signer list
- [x] query threshold
- [x] query whitelist-addresses
- [x] query transfer limit
- [x] query tx executions

# Deploy
1. Build
```
beaker wasm build --no-wasm-opt
```

2. Store code
```
 beaker wasm store-code account-management --no-wasm-opt --network pion --signer-mnemonic "mnemmonic"
```
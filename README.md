# Cosmpass Core Contracts

This repository contains core contracts for Cosmos Ecosystem DEXes.

## Osmosis

Due to the Liquidity Provisioning specifics in Osmosis - the following procedure is implemented for auto-compounding:

### Pool-specific Vault creation:

To enable autocompounding user needs to instantiate a pool-specific isolated vault smart-contract. This would allow user to have full control over deposited funds and isolate any risks associated with deposited assets.

```
TODO: add example vault-creation msg
```

### Deposit funds, Join pool
After the vault is created - user is able to deposit funds, join the pool and choose the unbonding period.

```
TODO: add example deposit-funds msg
```

### Unbond funds
User must unbond the LP tokens to be able to withdraw deposited funds

```
TODO: add example unbond-funds msg
```

### Withdraw funds
To withdraw funds:

```
TODO: add example withdraw-funds msg
```

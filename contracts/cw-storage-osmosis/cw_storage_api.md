# Documentation

This API document can be used in testing purposes.

First of all we need to set up Contract address in a variable, for convenience:

```sh
CONTRACT_ADDR='osmo16hjln5cvs0magddmzheqfljeq2s5wwuf2pe37a269fv98evep3dq6tj246'
```
Above is the address, for this contract that is stored and instantiated on Testnet of Osmosis (osmo-testnet-4).
Using this address we can query (get information) or execute messages (perform operations).

# Queries
In this version of the contract (0.1.0) there are 2 types of Queries that we can make:

1) Query to get information about all available pools.
First we need to write QueryMsg, then make call using it and previously written Contract address:

```sh
QUERY='{"query_pools":{}}'
osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json
```

2) Query to get information about user specific pools (pools where user have assets).
To make this query we need to specify user address:

```sh
QUERY='{"query_list_entries":{"user":"UserAddress"}}'
osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json
```

# Examples

1) For 1st Query:

```sh
QUERY='{"query_pools":{}}'
osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json
```

Output:

```sh
{"data":{"pools":[
    {"pool_id":"1","token_a_name":"osmo","token_b_name":"atom","token_a_addr":"0x818618433dff975f934cb85c35f8768d8ecec870","token_b_addr":"0x0eb3a705fc54725037cc9e008bdede697f62f335","apr":"19.81","apy":"43.15","daily_apr":"0.0983","tvl":"54201.13","converted_tvl":"54201.14","reward_coin":["osmo18l247apx8uwhg6kzxyuu2nre6n5zkpfate55pf"]},
    
    {"pool_id":"678","token_a_name":"usdc","token_b_name":"osmo","token_a_addr":"0x818618433dff975f934cb85c35f8768d8ecec870","token_b_addr":"0x0eb3a705fc54725037cc9e008bdede697f62f335","apr":"44.56","apy":"120.15","daily_apr":"2.983","tvl":"24501.13","converted_tvl":"24501.14","reward_coin":["osmo18l247apx8uwhg6kzxyuu2nre6n5zkpfate55pf"]}
    ]}}
```

2) For 2nd Query:
*This is an address of account, that is used in testing purposes:

```sh
QUERY='{"query_list_entries":{"user":"osmo18l247apx8uwhg6kzxyuu2nre6n5zkpfate55pf"}}'
osmosisd query wasm contract-state smart $CONTRACT_ADDR "$QUERY" --output json
```

Output:

```sh
{"data":{"entries":[{"id":1,"pool_id":"100","amount":"50000","pool_addr":"osmo100"},{"id":2,"pool_id":"101","amount":"45000","pool_addr":"osmo101"}]}}
```  
### Token Contract

Test uses : Use env.mock_all_auths() for simple testing (allows all auth)

![image](https://github.com/user-attachments/assets/0649f288-75a7-42e3-a174-33efd5e66668)

### Understanding Token Decimals in Soroban
The decimal value (18 in your code) determines the divisibility of your token. 
It represents the number of decimal places that your token can be divided into.

```Common Decimal Values
0 - No decimals (whole numbers only)
6 - Common for stablecoins (like USDC)
7 - Native XLM decimal places
8 - Common for Bitcoin-like tokens
18 - Common for Ethereum-like tokens
```

Examples with Different Decimals
```Decimal: 2
1 token = 100 base units
Smallest unit: 0.01
```

```Decimal: 7 (XLM standard)
1 token = 10,000,000 base units
Smallest unit: 0.0000001
```

```Decimal: 18 (Your current setting)
1 token = 1,000,000,000,000,000,000 base units
Smallest unit: 0.000000000000000001
```


#### How to Choose
Consider these factors when selecting decimals:
  -Use case of your token
  -Industry standards
  -Gas efficiency (smaller decimals = less computation)
  -User experience requirements

Example Code with Different Decimals
```// For a whole number token
client.initialize(&admin, &0, &symbol, &token);
```

```// For an XLM-like token
client.initialize(&admin, &7, &symbol, &token);
```
```// For a stablecoin-like token
client.initialize(&admin, &6, &symbol, &token);
```


Important Notes:
The decimal value cannot be changed after initialization
Choose carefully based on your token's intended use
More decimals means higher precision but also more complexity
Consider gas costs when working with large decimal values


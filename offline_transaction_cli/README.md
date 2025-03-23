# CLI application to generate blockchain transaction 
This CLI application is able to generate blockchain transaction 
on the **offline** environment.
> [!CAUTION]  
> When you build this CLI application, Cargo needs to access internet
> to collect dependencies.

## Build CLI Application
You can build this to use Cargo.
```shell
cargo build --release
```

## Usage
This CLI application has 2 argument (except for `-h` and `-V`). 
 - `-i`, `--input`  
   Input JSON file path which has the transaction data. 
   The JSON template is following this section.
 - `-o`, `--output` (**Optional**)  
   Output file path if you specify this argument, the hex transaction 
   will be written in the specified file and not display on the Stdout.  
   If you don't specify this argument, the transaction will be displayed 
   on Stdout.
```shell
offline_transaction -i <input_json_path> [-o <output_file>] 
```

## Input file template
```json
{
  "network": "bitcoin",
  "inputs": [
    {
      "txid": "<txid>",
      "vout": 0,
      "amount": 0.0,
      "address": "<source address>"
    }
  ],
  "outputs": [
    {
      "address": "<distance address>",
      "amount": 0.0
    }
  ],
  "changeAddress": "<change distance address (generally, the same as the source address)>",
  "privateKey": "<private key of the source address>",
  "feeRate": 1
}
```
### Arguments
 - `network`: The target blockchain network (currently, support only "bitcoin")
 - `inputs`: vector of the input transaction (1 or more transaction can be specified)
   - `txid`: UTXO transaction which is source of the transfer.
   - `vout`: Output index of the specified txid
   - `amount`: UTXO balance relates to the txid output
   - `address`: Address having the UTXO
 - `outputs`: vector of the output transaction (1 or more transaction can be specified)
   - `address`: Address which is the distance of the transaction
   - `amount`: Output amount which goes to the specified address
 - `changeAddress`: Change amount distance address
 - `privateKey`: Private key to sign the transaction which belong to the input addresses
 - `feeRate`: TransactionFee rate (sat/bytes)
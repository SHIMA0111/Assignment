# OfflineAssignment
This repository is for the offline assignment of the job.  

# Overview
This crate and CLI application is for generating blockchain
transaction on the **offline** environment.  
After generate a transaction, you can broadcast the transaction
from your online environment.  
For example, you may be able to use  
```shell
bitcoin-cli sendrawtransaction '<generated transaction>'
```

# Usage
This repository has 2 crates: 
 - `offline_transaction`: Library crates which has the core logics.
 - `offline_transaction_cli`: Binary crates which is CLI application to generate raw transaction.

If you want to use CLI application, please build the application. 
```shell
cargo build --release
```

Please clone and set your Cargo.toml if you want to build your own application.
You can use it as library, and you can build the crate documentation.
```shell
cargo doc
```
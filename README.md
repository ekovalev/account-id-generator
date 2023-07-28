# Vara random AccountIds generator


## Overview

A tool that generates dummy 32-byte AccountIDs for Vara network (encoded in Ss58 format using the prefix value 137) that are guaranteed to have no associated private key, hence not posing any risk should these AccountIds become known to anyone.

<br/>

### Usage examples

* Clone the repo

    ```bash
    git clone git@github.com:ekovalev/account-id-generator.git
    cd account-id-generator
    ```

* Build the binary

    ```bash
    cargo build --release
    ```

* Run the CLI

    ```bash
    ./target/release/account-id-generator [OPTIONS]
    ```


Options:
```
--prefix <PREFIX>  Unique prefix to ensure we don't occasionally create account with a private key [default: dmmy//mltsig//id]
-n, --n <N>        Number of samples [default: 10]
-h, --help         Print help
```

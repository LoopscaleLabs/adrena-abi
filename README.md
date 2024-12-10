# Adrena-ABI

The abi is a repository for interfacing with the Adrena program either through a rust client or through CPIs from another Solana program.

## Usage examples

`adrena-abi` can be used just like any other project built with `anchor-lang`.

### Build

`$>  cargo build -p adrena-abi`


### On the SG dedicated

Build `$> cargo build --release`

Launch daemon `$>daemon --name=mrsablier --output=~/service/MrSablier/logfile.log -- ~/services/MrSablier/target/release/mrsablier --payer-keypair /home/ubuntu/.config/solana/id.json --endpoint https://adrena.rpcpool.com/ --x-token <> --commitment processed`

Check if running `$>daemon --list`

Monitor live `$>tail -f logfile.log `
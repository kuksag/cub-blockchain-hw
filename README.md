# Blockchain course home tasks

## HW1

### How to use
```shell
$ cargo run -- --help

blockchain [SUBCOMMAND]

OPTIONS:
-h, --help       Print help information
-V, --version    Print version information

SUBCOMMANDS:
decrypt     Decrypt content of a .txt file using private key
encrypt     Encrypt content of a .txt file using public key
gen-keys    Generate public/private key pair
help        Print this message or the help of the given subcommand(s)
```

### Complete example

```shell
cargo run -- gen-keys --output key
cat key_public.pem
echo 'hello world' > input.txt
cargo run -- encrypt --input input.txt --output encrypted.txt --key key_public.pem
cargo run -- decrypt --input encrypted.txt --output decrypted.txt --key key_private.pem
diff input.txt decrypted.txt
```
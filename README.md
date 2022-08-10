# exchange-cache

## Run in ubuntu

Compile openssl 1.1.1

- `sudo apt update`
- `sudo apt install build-essential checkinstall zlib1g-dev -y`
- `sudo wget https://www.openssl.org/source/openssl-1.1.1q.tar.gz`
- `sudo tar -xf openssl-1.1.1q.tar.gz`
- `cd d openssl-1.1.1q`
- `sudo ./config --prefix=/usr/local/ssl_1.1.1 --openssldir=/usr/local/ssl_1.1.1 shared zlib`
- `sudo make`
- `sudo make test`
- `sudo make install`
- Run this always before building `export OPENSSL_DIR=/usr/local/ssl_1.1.1/`
- Build and run `cargo run --bin ${target}`

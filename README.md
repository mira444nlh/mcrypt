# mcrypt

A simple command-line file encryptor written in Rust.

`mcrypt` encrypts files using a password-derived key with **Argon2id** and authenticated encryption with **XChaCha20-Poly1305**.

## Features

* 🔐 Argon2id password-based key derivation
* 🔒 XChaCha20-Poly1305 authenticated encryption
* 📁 File encryption and decryption
* 🦀 Written in Rust
* 🖥️ Simple CLI interface

## Usage

### Encrypt

```bash
mcrypt encrypt input.txt
```

Specify an output file:

```bash
mcrypt encrypt input.txt -o encrypted.bin
```

### Decrypt

```bash
mcrypt decrypt encrypted.bin
```

Specify an output file:

```bash
mcrypt decrypt encrypted.bin -o decrypted.txt
```

### Options

```text
Usage: mcrypt [OPTIONS] <COMMAND>

Commands:
  encrypt  Encrypt a file
  decrypt  Decrypt a file

Options:
  -s, --silent  Hide password input feedback
  -h, --help    Print help
  -V, --version Print version
```

## Installation

### Prebuilt binaries

**macOS and Linux:**

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/mira444nlh/mcrypt/releases/latest/download/mcrypt-installer.sh | sh
```

**Windows (PowerShell):**

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/mira444nlh/mcrypt/releases/latest/download/mcrypt-installer.ps1 | iex"
```

Alternatively, download the archive for your platform from the [releases page](https://github.com/mira444nlh/mcrypt/releases), extract it, and put `mcrypt` somewhere on your `PATH`.

### Homebrew

```sh
brew install mira444nlh/tap/mcrypt
```

### Build from source

Requires the [Rust toolchain](https://rustup.rs/).

```sh
cargo install --git https://github.com/mira444nlh/mcrypt
```

### Verify

```sh
mcrypt --version
```

## Cryptography

The encryption key is derived from the user's password using **Argon2id**.

The derived key is then used with **XChaCha20-Poly1305** to encrypt and authenticate the file contents.

Each encrypted file contains the parameters required for decryption, including a randomly generated salt and nonce.

> This project is primarily intended as a learning project and should not be relied upon for protecting highly sensitive data without independent security review.

## License

See [LICENSE](LICENSE).

# passgen

Generate secure passwords from the terminal.

## Install

```console
cargo build --release
sudo cp target/release/passgen /usr/local/bin/
```

## Usage

```console
passgen
passgen -l 32 -o password.txt
```

Output:

```
xK9mP2vQ7nL4wR8jY3hF6tG1zS5aB0cD
```

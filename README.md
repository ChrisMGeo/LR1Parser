# LR(1) Parser Generator written in Rust
LR(1) Parser Generator written in Rust. Takes in a grammar and outputs a parsing table, which can be displayed.

> Warning: This project is still in development and may not work as expected.
> The current output is very large and may need to be redirected to a file to be viewed.
```bash
git clone https://github.com/ChrisMGeo/LR1Parser.git
cd LR1Parser
# Redirect the below command with support for unicode characters
cargo run --release
# might have to use the following in Windows before the above command in case border characters are not supported
[Console]::OutputEncoding = [System.Text.Encoding]::new($false)
```

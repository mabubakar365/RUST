RUST

sudo apt-get install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version

cargo new hello
cd hello
cargo build 
cargo run

git init
git add .
git commit -m "enter your message here"
git remote add "enter your repo path here"
git push -u origin master


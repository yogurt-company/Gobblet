install:
	sudo apt-get update
	sudo apt-get install -y build-essential
  	sudo apt-get install -y libtorch
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source $HOME/.cargo/env
  	cargo build

apple_silicon_install:
	sudo apt-get update
	sudo apt-get install -y build-essential
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	source $HOME/.cargo/env
  	cargo build
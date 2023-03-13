# Gobblet
Gobblet  + Reinforcement learning. 

# Installation 
For apple silicon m1/m2 users, you need to install the following packages first:


`brew install libtensorflow` for tensorflow install

`brew install libtorch` and add 
```bash
export LIBTORCH=/path/to/libtorch
export LD_LIBRARY_PATH=${LIBTORCH}/lib:$LD_LIBRARY_PATH
```
to ~/.profile (any equivalent file). Default terminal for mac user is zsh, that is ,
`~/.zshrc` 

# Quick experiment
we recommend to use codespace to quick test file.  You only need to login into codespace and run following command to install.
```bash
make install
```


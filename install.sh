#!/bin/bash

if [[ $1 == '--uninstall' ]]; then
    rm -rf $HOME/.local/bin/kx $HOME/.local/bin/kn
    echo "Uninstall kn and kx from $HOME/.local/bin/kn $HOME/.local/bin/kx"
    exit
fi

if [[ $(uname) == "Linux" ]]; then
    cd /tmp
    tag_name=$(curl -s "https://api.github.com/repos/koolwithk/kx-kn-rust/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    curl -s https://github.com/koolwithk/kx-kn-rust/releases/download/$tag_name/kn-$tag_name-x86_64-linux-musl.tar.gz -L --output kn-$tag_name-x86_64-linux-musl.tar.gz
    tar -xzf kn-$tag_name-x86_64-linux-musl.tar.gz
    rm -rf kn-$tag_name-x86_64-linux-musl.tar.gz
    mkdir -p $HOME/.local/bin
    /usr/bin/mv kn $HOME/.local/bin

    curl -s https://github.com/koolwithk/kx-kn-rust/releases/download/$tag_name/kx-$tag_name-x86_64-linux-musl.tar.gz -L --output kx-$tag_name-x86_64-linux-musl.tar.gz
    tar -xzf kx-$tag_name-x86_64-linux-musl.tar.gz
    rm -rf kx-$tag_name-x86_64-linux-musl.tar.gz
    /usr/bin/mv kx $HOME/.local/bin
    echo "Installed kn and kx $tag_name at $HOME/.local/bin/kx $HOME/.local/bin/kn"

    if [[ "$(echo "$PATH" | grep -o "$HOME/.local/bin")" != "$HOME/.local/bin" ]]; then
        echo 'Please add /usr/local/bin to your $PATH'
    fi
else
    echo "$(uname) OS not supported by this script :("
fi
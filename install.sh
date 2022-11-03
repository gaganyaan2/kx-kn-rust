#!/bin/bash

if [ "$1" = '--uninstall' ]; then
    rm -rf $HOME/.local/bin/kx $HOME/.local/bin/kn
    echo "Uninstall kn and kx from $HOME/.local/bin/kn $HOME/.local/bin/kx"
    exit
fi

#### check curl
if ! [ -x "$(command -v curl)" ]; then
    echo 'Error: curl is not installed.' >&2
    exit 1
fi

installed_msg() {
    echo ''
    echo "Installed kn and kx ${TAG_NAME} at $HOME/.local/bin/kx $HOME/.local/bin/kn"
        echo ''
        echo '    Please add "$HOME/.local/bin" to your $PATH'
        echo '    PATH="$PATH:$HOME/.local/bin"'
        echo ''
}

install_kxkn() {
    TAG_NAME=$1
    OS=$2
    ARCH=$3
    KXKN=$4
    #kn-v0.1.0-linux-arm64.tar.gz
    cd /tmp
    curl -s https://github.com/koolwithk/kx-kn-rust/releases/download/${TAG_NAME}/${KXKN}-${TAG_NAME}-${OS}-${ARCH}.tar.gz -L --output ${KXKN}-${TAG_NAME}-${OS}-${ARCH}.tar.gz
    tar -xzf ${KXKN}-${TAG_NAME}-${OS}-${ARCH}.tar.gz
    rm -rf ${KXKN}-${TAG_NAME}-${OS}-${ARCH}.tar.gz
    mkdir -p $HOME/.local/bin
    mv ${KXKN} $HOME/.local/bin
}

#### check Architecture
if [ "$(uname -m | grep -o aarch64)" = "aarch64" ] || [ "$(uname -m | grep -o arm64)" = "arm64" ]; then
    arch=arm64
elif [ "$(uname -m | grep -o x86_64)" = "x86_64" ] || [ "$(uname -m | grep -o amd64)" = "amd64" ] ; then
    arch=x86_64
else
    echo "$(uname -m) $(uname) OS not supported by this script :("
fi

tag_name=$(curl -s "https://api.github.com/repos/koolwithk/kx-kn-rust/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ "$(uname)" = "Linux" ] && [ "x86_64" = "$arch" ] ; then
    os="linux"
    install_kxkn "$tag_name" "$os" "$arch" "kx"
    install_kxkn "$tag_name" "$os" "$arch" "kn"
    installed_msg
elif [ "$(uname)" = "Linux" ] && [ "arm64" = "$arch" ] ; then
    os="linux"
    install_kxkn "$tag_name" "$os" "$arch" "kx"
    install_kxkn "$tag_name" "$os" "$arch" "kn"
    installed_msg
elif [ "$(uname)" = "Darwin" ] && [ "arm64" = "$arch" ] ; then
    os="darwin"
    install_kxkn "$tag_name" "$os" "$arch" "kx"
    install_kxkn "$tag_name" "$os" "$arch" "kn"
    installed_msg
    echo 'for Apple Mac follow https://github.com/koolwithk/kx-kn-rust/issues/6 OR Build your own binary!!!'
    echo ''
else
    echo "$(uname -m) $(uname) OS not supported by this script :( Please install from release page."
fi
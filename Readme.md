# kx-kn - Simple kubernetes context and namespace switch

[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)

- **kx** - switch context between kubernetes
- **kn** - switch namespace between kubernetes

### Install kx and kn on linux:

```bash
curl -s https://raw.githubusercontent.com/koolwithk/kx-kn-rust/main/install.sh | bash
```

### `kxkn` cli usage
```bash
kn                 #show current namespace
kx                 #show current context

kn <namespace>     #switch to other namespace
kx <context>       #switch to other context
```

### Build binary

```
git clone https://github.com/koolwithk/kx-kn-rust.git
cd kx
cargo build --release

## for statically linked binary
cargo build --target=x86_64-unknown-linux-musl  --release
```

### Why kx and kn in rust?
- learn rust
- small binary size and fast

### Refrences:
- https://github.com/ahmetb/kubectx [Inspired By]
- https://doc.rust-lang.org/rust-by-example/index.html
- https://docs.rs/kube/latest/kube/config/index.html
- https://docs.rs/kube-conf/latest/kube_conf/
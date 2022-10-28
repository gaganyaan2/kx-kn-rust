# kx-kn - Simple kubernetes context and namespace switch

- **kx** - switch context between kubernetes
- **kn** - switch namespace between kubernetes

### Install kx and kn in linux:

```bash
curl -s https://raw.githubusercontent.com/koolwithk/kx-kn-rust/main/install.sh | bash
```

### Run:
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
- small binary size
- fast

### Refrences:
- https://github.com/ahmetb/kubectx (Inspired By)
- https://doc.rust-lang.org/rust-by-example/index.html
- https://docs.rs/kube/latest/kube/config/index.html
- https://docs.rs/kube-conf/latest/kube_conf/
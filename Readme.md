# kx-kn in rust
Simple kubernetes context and namespace switch in rust

- **kx** - Switch between kubernetes context
- **kn** - Switch between kubernetes namespace

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

### Why kx and kn in rust?
- small binary size
- fast

### Refrences:
- https://github.com/ahmetb/kubectx (Inspired By)
- https://doc.rust-lang.org/rust-by-example/index.html
- https://docs.rs/kube/latest/kube/config/index.html
- https://docs.rs/kube-conf/latest/kube_conf/
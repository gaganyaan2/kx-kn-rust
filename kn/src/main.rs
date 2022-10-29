mod kubeconfig;
mod kn;
mod flags;

fn main(){
  let conf = kubeconfig::get_kubeconfig_file();
  let k_namespace = flags::flags();
  kn::kn(&conf, &k_namespace);
}
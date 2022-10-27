mod kubeconfig;
mod kn;
mod flags;
fn main(){
  let conf = kubeconfig::get_kubeconfig_file();
  flags::flags();
  kn::kn(&conf);
}
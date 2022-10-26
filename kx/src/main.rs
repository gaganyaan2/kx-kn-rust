mod kubeconfig;
mod kx;
mod flags;
fn main(){
  let conf = kubeconfig::get_kubeconfig_file();
  flags::flags();
  kx::kx(&conf);
}
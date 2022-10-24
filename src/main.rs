mod kubeconfig;
mod kx;
fn main(){
  let conf = kubeconfig::get_kubeconfig_file();
  kx::kx(&conf);
}
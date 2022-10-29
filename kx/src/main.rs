mod kubeconfig;
mod kx;
mod flags;

fn main(){
  let kconf = kubeconfig::get_kubeconfig_file();
  let kcontext = flags::flags();
  kx::kx(&kconf, &kcontext);
}
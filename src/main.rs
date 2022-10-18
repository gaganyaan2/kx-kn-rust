mod kubeconfig;

fn main(){
  kubeconfig::kubeconfig_from_env();
  kubeconfig::kubeconfig_from_home_dir();
}
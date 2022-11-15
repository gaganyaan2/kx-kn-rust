_kx()
{
  local curr_arg;
  curr_arg=${COMP_WORDS[COMP_CWORD]}
  COMPREPLY=( $(compgen -W "$(kx)" -- $curr_arg ) );
}

complete -F _kx kx
_kn()
{
  local curr_arg;
  curr_arg=${COMP_WORDS[COMP_CWORD]}
  COMPREPLY=( $(compgen -W "$(kn)" -- $curr_arg ) );
}

complete -F _kn kn
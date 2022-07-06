A="hello::world::user"
arr=($(echo $A | tr "::" "\n"))

echo ${arr[-1]}

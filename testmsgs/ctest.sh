# requires a symbolic link to the msgs C program!  for example:
# ln -s ~/code/arduino/automato-library/tests/msgs

# generate messages from C.

mkdir cmsgs-out
./msgs out cmsgs-out

# read with rust

./target/debug/testmsgs -d cmsgs-out read

# write rust msgs
mkdir rust-out
./target/debug/testmsgs -d rust-out write

# read with C.

./msgs in rust-out

# requires a symbolic link to the msgs C program!  for example:
# ln -s ~/code/arduino/automato-library/tests/msgs

if [ -f 'msgs' ]; then
   echo "File msgs exists."
else
   echo "File 'msgs' does not exist.  Build the program in automto-library/tests, then make a link to it like this:"
   echo "ln -s ~/mycode/automato-library/tests/msgs"
   exit 1
fi

# generate messages from C.

echo "----------------------------------------------------------------"
echo "writing C messages"
echo "----------------------------------------------------------------"

mkdir -p cmsgs-out
./msgs out cmsgs-out

if [ $? -ne 0 ]; then
   echo "cmsgs-out failed!"
   exit 1
fi

# read with rust

echo "----------------------------------------------------------------"
echo "reading C messages with rust"
echo "----------------------------------------------------------------"

./target/debug/testmsgs -d cmsgs-out read

if [ $? -ne 0 ]; then
   echo "testmsgs read failed!"
   exit 1
fi

# write rust msgs
echo "----------------------------------------------------------------"
echo "writing messages from rust"
echo "----------------------------------------------------------------"

mkdir -p rust-out
./target/debug/testmsgs -d rust-out write

if [ $? -ne 0 ]; then
   echo "testmsgs write failed!"
   exit 1
fi


# read with C.

echo "----------------------------------------------------------------"
echo "reading rust messages from C"
echo "----------------------------------------------------------------"

./msgs in rust-out

if [ $? -ne 0 ]; then
   echo "testmsgs write failed!"
   exit 1
fi

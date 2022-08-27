
# write json msgs with rust

# echo "----------------------------------------------------------------"
# echo " writing json messages with rust"
# echo "----------------------------------------------------------------"

# mkdir -p json-msgs
# ./target/debug/testmsgs -d json-msgs --json write

# if [ $? -ne 0 ]; then
#    echo "testmsgs write failed!"
#    exit 1
# fi

# read json msgs
echo "----------------------------------------------------------------"
echo "reading json messages from rust"
echo "----------------------------------------------------------------"

./target/debug/testmsgs -d json-msgs --json read

if [ $? -ne 0 ]; then
   echo "testmsgs read failed!"
   exit 1
fi



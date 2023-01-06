g++ -c *.cpp
gcc -c *.c
g++ *.o -lglfw -o out
rm *.o
./out
rm out

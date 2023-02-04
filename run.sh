mkdir -p build
cd build
CMAKE_GENERATOR=Ninja cmake .. >/dev/null # just so that it shuts up
ninja
cp src/out ..
cd ..
./out

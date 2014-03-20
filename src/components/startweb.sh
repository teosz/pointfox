echo 'Starting  server on http://localhost:'$1
rustc -O -Z debug-info initweb.rs -o run -L ../http/build/
./run

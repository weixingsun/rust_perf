cargo build --release
tplist -l target/release/test1

target/release/test1 &
sudo ./rust_perf.py -n test1 -b loop -i 1 -d 5

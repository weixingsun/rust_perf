pkill test1
cargo build --release
tplist -l target/release/test1

target/release/test1 &
sleep 2
sudo ../rust_perf -n test1 -b loop -i 1 -d 5 > loop.out &
sudo ../rust_perf -n test1 -b add -i 1 -d 5 > add.out &
sudo ../rust_perf -n test1 -b sub -i 1 -d 5 > sub.out &
sudo ../rust_perf -n test1 -b sleep -i 1 -d 5 > sleep.out &   # ['sleep__start', 'sleep__end', 'sleep__start', 'sleep__end']

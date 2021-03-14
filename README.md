# Performance measurement for Rust

As a known issue, officially there is still no tracing point support for rust stable (std). [PR](https://github.com/rust-lang/rust/pull/14031)

But USDT tracing is widely used in almost everywhere in performance measurement, analysis and optimization.

Luckily we have a module [libprobe](https://github.com/cuviper/rust-libprobe) as a workaround

## Preparation

Cargo.toml

```toml
[dependencies.probe]
git = "https://github.com/cuviper/rust-libprobe.git"
```
source code changes
```rust
#![feature(asm)]                                     // need asm lib

use probe::probe;
use std::{thread, time};

fn add(a: u64, b: u64) -> u64 {
    probe!(test1, add__start, a, b);                 // added for function entry
    thread::sleep(time::Duration::from_millis(10));
    probe!(test1, add__end, a, b);                   // added for function exit
    return a+b;
}
```

## Build

```bash
cargo build --release
```

## Check

```bash
$ tplist -l target/release/test1
b'target/release/test1' b'test1':b'add__start'
b'target/release/test1' b'test1':b'add__end'

$ readelf -n ./target/release/test1 | grep NT_STAPSDT -B2 -A4
readelf: Warning: Gap in build notes detected from 0x518a to 0x35c0f
Displaying notes found in: .note.stapsdt
  Owner                Data size 	Description
  stapsdt              0x00000039	NT_STAPSDT (SystemTap probe descriptors)
    Provider: test1
    Name: add__start
    Location: 0x00000000000051dc, Base: 0x000000000003aa41, Semaphore: 0x0000000000000000
    Arguments: -8@%r14 -8@%r12
  stapsdt              0x00000037	NT_STAPSDT (SystemTap probe descriptors)
    Provider: test1
    Name: add__end
    Location: 0x00000000000051e7, Base: 0x000000000003aa41, Semaphore: 0x0000000000000000
    Arguments: -8@%r14 -8@%r12

```


## Run the target rust app

```bash
target/release/test1 &
```

## Run rust_perf tool

```bash
$ sudo ./rust_perf -n test1 -b add -i 1 -d 2 
Tracing function test1.add functions ... Hit Ctrl-C to end.

16:23:31
     msecs               : count     distribution
         0 -> 1          : 0        |                                        |
         2 -> 3          : 0        |                                        |
         4 -> 7          : 0        |                                        |
         8 -> 15         : 33       |****************************************|

avg = 10 msecs, total: 332 msecs, count: 33


16:23:32
     msecs               : count     distribution
         0 -> 1          : 0        |                                        |
         2 -> 3          : 0        |                                        |
         4 -> 7          : 0        |                                        |
         8 -> 15         : 33       |****************************************|

avg = 10 msecs, total: 665 msecs, count: 66
```
## Multiple tracing points

```bash
BIN="test1"
target/release/$BIN &
sudo python ./rust_perf -n $BIN -i 1 -d 5 -b add > add.out &
sudo python ./rust_perf -n $BIN -i 1 -d 5 -b sub > sub.out &
sudo python ./rust_perf -n $BIN -i 1 -d 5 -b loop > loop.out &

```
Throughput matrix (RPS)
```bash
             fn_loop      fn_add       fn_sub
16:23:31     33           99           49
16:23:32     33           99           49
```
Latency matrix (ms)
```bash
             fn_loop      fn_add       fn_sub
16:23:31     30           10           20
16:23:32     30           10           20
```

## Why not just writing logs

For the light-weight apps may be straight forward, but for a high throughput system, dealing with every IO operation(disk/network) should be cautious

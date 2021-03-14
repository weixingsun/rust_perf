#![feature(asm)]

use probe::probe;
use std::{thread, time};
fn sleep(a: u64){
    probe!(test_2, sleep__start, a);
    thread::sleep(time::Duration::from_millis(a));
    probe!(test_2, sleep__end, a);
}
fn add(a: u64, b: u64) -> u64 {
    probe!(test_1, add__start, a, b);
    sleep(10);
    probe!(test_1, add__end, a, b);
    return a+b;
}
fn sub(a: u64, b: u64) -> u64 {
    probe!(test_1, sub__start, a);
    sleep(20);
    probe!(test_1, sub__end, a);
    return a-b;
}

fn infinite_loop(n: u64){
    let mut a: u64 = 10;
    for i in 1..=n {
        probe!(test_0, loop__start);
        a = add(a, i);
        //sleep(a);
        a = sub(a, i);
        probe!(test_0, loop__end);
    }
}

fn main() {
    infinite_loop(10000);
}

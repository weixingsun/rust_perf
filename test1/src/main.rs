#![feature(asm)]

use probe::probe;
use std::{thread, time};

fn add(a: u64, b: u64) -> u64 {
    probe!(probe, add__start, a, b);
    thread::sleep(time::Duration::from_millis(10));
    probe!(probe, add__end, a, b);
    return a+b;
}
fn sub(a: u64, b: u64) -> u64 {
    probe!(probe, sub__start, a);
    thread::sleep(time::Duration::from_millis(20));
    probe!(probe, sub__end, a);
    return a-b;
}

fn infinite_loop(n: u64){
    let mut a: u64 = 10;
    //let delay = time::Duration::from_millis(i);
    for i in 1..=n {
        probe!(probe, loop__start);
        a = add(a, i);
        //thread::sleep(delay);
        a = sub(a, i);
        probe!(probe, loop__end);
    }
}

fn main() {
    infinite_loop(1000);
}

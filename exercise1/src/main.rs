// 数字を出力しつつ、入力待機はあきらめた
// Q以外のキーを押すとカウントが進む
// Qを押すと終了

extern crate getch;

use getch::Getch;
use std::thread;
use std::sync::mpsc;

fn main() {
    println!("Press Q to quit.");
    let (port, chan) = mpsc::channel();

    thread::spawn(move || {
        let key = Getch::new();
        loop {
            port.send(key.getch().unwrap()).unwrap();
        }
    });

    let mut num = 0;
    loop {
        println!("{}", num);
        num += 1;
        if chan.recv().unwrap() == 113 {
            break
        }
    }
}
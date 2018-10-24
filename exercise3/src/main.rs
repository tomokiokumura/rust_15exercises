// 文字列を配列に入れる方法よくわからないので数字だけで実装

// 文字列の配列を作るときは&str型じゃないと作れないっぽい
// stdinはString型で入力を受け取る
// String型は&varで&str型に変換できるらしいが配列に入れられなかった

fn main() {
    let mut num = [0; 10];
    println!("Please input 10 numbers.");
    for i in 0..10 {
        num[i] = read();
    }

    num.sort();
    println!("\nASC");
    for i in 0..10 {
        println!("{}", num[i]);
    }

    num.reverse();
    println!("\nDESC");
    for i in 0..10 {
        println!("{}", num[i]);
    }
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}
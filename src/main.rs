use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数を当ててみましょう");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("予想を入力してください");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("行の読み込みに失敗しました");

    let guess: u32 = guess.trim().parse()
                    .expect("数値を入力してください");
    println!("次のように予想しました {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("正解よりも小さいです"),
        Ordering::Greater => println!("正解よりも大きいです"),
        Ordering::Equal => println!("正解です"),
    }
}

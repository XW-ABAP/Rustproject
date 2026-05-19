use std::cmp::Ordering;
use std::io;
use rand::Rng;
// 引入必要的标准库模块来控制时间和线程休眠
use std::thread;
use std::time::Duration;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) 
        {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // 增加一句提示，让用户知道正在等待
                println!("程序将在 3 秒后自动退出...");
                // 让当前线程休眠 3 秒
                thread::sleep(Duration::from_secs(3));
                break;
            }
        }
    }
    
}

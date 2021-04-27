fn main() {
    let n = 20;
    println!("The fibonacci number at position {} is {}", n, fibonacci(n));
}

fn fibonacci(x : i64) -> i64{
    let mut final_num = 0;
    let mut two_ago = 0;
    let mut one_ago = 0;
    for count in 0..x + 1{
        let current_number = if count < 2 { count } else{ one_ago + two_ago };
        final_num = current_number;
        two_ago = one_ago;
        one_ago = current_number;
        println!("{}", current_number);
    }
    final_num
}

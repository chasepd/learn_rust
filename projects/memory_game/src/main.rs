use rand::{thread_rng, Rng};

fn select_next_number (mut number_history : [i32; 10]) -> [i32; 10]{
    let mut rng = thread_rng();
    let new_number: i32 = rng.gen_range(0..100);
    number_history[1] = new_number;
    return number_history;
}

fn main() {
    let mut number_history : [i32; 10] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    number_history = select_next_number(number_history);
    for n in number_history.iter(){
        println!("{}", n);
    }
}

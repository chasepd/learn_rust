fn main() {
    let x = five() * 5;
    let y = {
        let x = 6;
        x + 1
    };
    function_2(x, y);
}

fn function_2(x: i32, y: i32){
        println!("The value of x is {}", x);
        println!("The value of y is {}", y);
        if x > y {
            println!("x is greater than y.");
        }
        else if y > x {
            println!("x is less than y.");
        }
        else{
            println!("x is equal to y.");
        }
}

fn five() -> i32 {
    let mut counter = 0;
    let x = loop{
        counter += 1;
        if counter == 5{
            break counter;
        }
    };
    x
}

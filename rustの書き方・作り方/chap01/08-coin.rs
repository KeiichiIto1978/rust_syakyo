fn coin(){
    let change = 3950;

    let num_of_500 = 10;
    let num_of_100 = 3;
    let num_of_50 = 10;


    for i500 in 0..num_of_500 + 1{
        for i100 in 0..num_of_100 + 1{
            for i50 in 0..num_of_50 + 1{
                let total = 500 * i500 + 100 * i100 + 50 * i50;
                if change == total {
                    println!("500円×{}枚、100円×{}枚、50円×{}枚", i500, i100, i50);
                }
            }
        }
    }
}

fn coin_type(){
    let change: i64 = 3950;

    let num_of_500: i64 = 10;
    let num_of_100: i64 = 3;
    let num_of_50: i64 = 10;


    for i500 in 0..num_of_500 + 1{
        for i100 in 0..num_of_100 + 1{
            for i50 in 0..num_of_50 + 1{
                let total: i64 = 500 * i500 + 100 * i100 + 50 * i50;
                if change == total {
                    println!("500円×{}枚、100円×{}枚、50円×{}枚", i500, i100, i50);
                }
            }
        }
    }
}    


fn main() {
    println!("coin");
    coin();
    println!("coint_type");
    coin_type();
}
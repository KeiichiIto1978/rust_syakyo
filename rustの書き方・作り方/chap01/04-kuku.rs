fn kuku(){
    for x in 1..10{
        for y in 1..10{
            print!("{:3},", x * y)
        }
        println!("")
    }
}

fn kuku2(){
    for y in 1..10{
        let s = (1..10)
            .map(|x| format!("{:3}", x * y))
            .collect::<Vec<String>>().join(",");
        println!("{}",s);
    }
}

fn main(){
    println!("kuku()");
    kuku();

    println!("kuku2()");
    kuku2();    
}
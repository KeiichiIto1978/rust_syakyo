fn main(){
    let pc_price = 98000.0;
    
    let a_ship_free =1200.0;
    let a_rate = 0.8;

    let b_ship_free = 0.0;
    let b_rate = 0.9;

    println!("A社{:4}円", pc_price * a_rate + a_ship_free);
    println!("B社{:4}円", pc_price * b_rate + b_ship_free);

}
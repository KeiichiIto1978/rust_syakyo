fn main(){
    for year in 1926..2027{
        print!("西暦{}年 = ", year);

        // 令和
        if year >= 2019  {
            if year == 2019 {println!("令和元年"); }
            else { println!("令和{}年", year - 2019 + 1); }
        }
        // 平成
        else if year >= 1989 && year < 2019 {
            if year == 1989 {println!("平成元年"); }
            else { println!("平成{}年", year - 1989 + 1); }
        }
        // 昭和
        else if year >= 1926 && year < 1989 {
            if year == 1926 {println!("昭和元年"); }
            else { println!("昭和{}年", year - 1926 + 1); }
        }
    }
}
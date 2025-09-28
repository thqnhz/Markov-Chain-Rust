fn main() {
    match gamblers_ruin(100, 1) {
        Ok(result) => println!("{}", result),
        Err(e) => println!("{}", e)
    }
}

fn gamblers_ruin(mut money: i32, bid: i32) -> Result<String, String> {
    if bid > money || money / bid > 1000 {
        Err("Bid is too big or too small".to_string())
    } else {
        let target_money: i32 = money * 2;
        let mut attempt: u16 = 1;
        while money > 0 && money < target_money {
            money += if rand::random_bool(0.5) {1} else {-1} * bid;
            attempt += 1;
        }
        let result = String::from(format!("After {} attempts, you ", attempt.to_string()));
        if money <= 0 {
            Ok(result + "lost it all at the casino! RIPBOZO")
        } else {
            Ok(result + "doubled your money! $$$")
        }
    }
}

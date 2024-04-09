pub fn number(user_number: &str) -> Option<String> {
    
    let only_number: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
    
    match only_number.len() {
        10 => {
            if check_ns(&only_number, "0") && check_ns(&only_number, "3") {
                return Some(only_number);
            }   
        }
        11 => {
            if only_number.starts_with('1') && check_ns(&only_number, "1") && check_ns(&only_number, "4")  {
                let sms_number = only_number[1..].to_string();
                return Some(sms_number);
            }       
        }
        _ => {}
    }
    None
}

fn check_ns(only_number: &str, x: &str) -> bool {
    if let Some(c) = only_number.chars().nth(x.parse().unwrap()) {
        return c != '0' && c != '1';
    }
    false
}

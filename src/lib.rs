pub fn number(user_number: &str) -> Option<String> {
    
    // Remove non digit character even whitespaces
    let mut only_number = String::new();
    for c in user_number.chars(){
        if c.is_ascii_digit(){
            only_number.push(c)
        }
    }

    match only_number.len() {
        10 => {
            if only_number.chars().nth(0) != Some('0') &&  only_number.chars().nth(0) != Some('1') && 
                only_number.chars().nth(3) != Some('0') && only_number.chars().nth(3) != Some('1') {
                    return Some(only_number)
                }   
        }
        11 => {
            if only_number.chars().nth(0) == Some('1') && only_number.chars().nth(1) != Some('0') && 
                only_number.chars().nth(1) != Some('1') && only_number.chars().nth(4) != Some('0') && 
                only_number.chars().nth(4) != Some('1') {
                    let sms_number = only_number[1..].to_string();
                    return Some(sms_number)
                }       
        }
        _ =>  {}
    }

    None
}

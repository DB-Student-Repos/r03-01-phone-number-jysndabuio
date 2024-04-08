pub fn number(user_number: &str) -> Option<String> {
    
    // Remove non digit character even whitespaces
    let only_number: String = user_number.chars().filter(|c| c.is_ascii_digit()).collect();
    

    match only_number.len() {
        10 => {
            // It workds but... the process is checking  the first 4 numbers if != 0 or 1. 
            if only_number[0..=3].chars().all(|x| x != '0' && x != '1') {
                    return Some(only_number)
                }   
        }
        11 => {
            if only_number.chars().nth(0) == Some('1') && only_number[1..=4].chars().all(|x| x != '0' && x != '1') {
                    let sms_number = only_number[1..].to_string();
                    return Some(sms_number)
                }       
        }
        _ =>  {}
    }

    None
}

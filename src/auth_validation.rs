

pub fn validate_aut(username:String,password1:String,password2:String,metod:i8)->bool{

    //metod 1 = login, metod 2 = register
    let mut _exit_test_register: bool = false;
    if metod == 1 {
        
    }
    else if metod == 2 {
        _exit_test_register = checker_registration(username, password1, password2);
    }
    else {
     println!("Eroror metod file auth_validation.rs!");
    }
    println!("{}",_exit_test_register);
    _exit_test_register

}


fn checker_registration(_username:String,_password1:String,_password2:String) -> bool {
        let mut check_error:bool  = false;
        
        if _password1.len() >= 8 {
    
       

            if _password1 == _password2 {
                
                if is_valid_password(&_password1) {
                    check_error = true;

                    
    
                }
            }
        }
 


        check_error
}





pub fn is_valid_password(password: &str) -> bool {
    let min_len = 8;
    let mut has_digit = false;
    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_special = false;

    for c in password.chars() {
        if c.is_ascii_digit() {
            has_digit = true;
        } else if c.is_ascii_uppercase() {
            has_upper = true;
        } else if c.is_ascii_lowercase() {
            has_lower = true;
        } else if !"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".contains(c) {
            has_special = true;
        }
    }

    // Пароль должен содержать минимум:
    // - 8 символов
    // - 1 цифру
    // - 1 заглавную букву
    // - 1 строчную букву
    // - 1 специальный символ
    password.len() >= min_len && has_digit && has_upper && has_lower && has_special
}

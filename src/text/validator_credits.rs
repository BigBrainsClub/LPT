use crate::config::Config;

#[inline(always)]
pub fn is_valid_phone_number(number: &[u8]) -> bool {
    let len = number.len();
    if len == 0 {
        return false;
    }

    unsafe {
        let mut ptr = number.as_ptr();
        let end = ptr.add(len);

        if *ptr == b'+' {
            ptr = ptr.add(1);
        }

        let mut has_digits = false;
        let mut open_parentheses = 0;

        while ptr < end {
            let b = *ptr;
            match b {
                b'0'..=b'9' => {
                    has_digits = true;
                }
                b'(' => {
                    if open_parentheses > 0 {
                        return false; // Только одни скобки допустимы
                    }
                    open_parentheses += 1;
                }
                b')' => {
                    if open_parentheses == 0 {
                        return false; // Закрывающая скобка без открывающей
                    }
                    open_parentheses -= 1;
                }
                b'-' | b'.' | b' ' => {
                    // Разделители допустимы
                }
                _ => {
                    return false; // Недопустимый символ
                }
            }
            ptr = ptr.add(1);
        }

        has_digits && open_parentheses == 0
    }
}


#[inline(always)]
pub fn is_valid_login(login: &[u8]) -> bool {
    let len = login.len();
    if len == 0 {
        return false;
    }

    unsafe {
        let mut ptr = login.as_ptr();
        let end = ptr.add(len);
        
        let first = *ptr;
        if !((first >= b'a' && first <= b'z') || 
             (first >= b'A' && first <= b'Z') || 
             (first >= b'0' && first <= b'9')) {
            return false;
        }
        
        let mut prev = first;
        ptr = ptr.add(1);
        
        while ptr < end {
            let b = *ptr;
            if !((b >= b'a' && b <= b'z') || 
                 (b >= b'A' && b <= b'Z') || 
                 (b >= b'0' && b <= b'9') || 
                 b == b'_' || b == b'-' || b == b'.') {
                return false;
            }
            
            if (b == b'.' || b == b'-' || b == b'_') && prev == b {
                return false;
            }
            
            prev = b;
            ptr = ptr.add(1);
        }
    }

    true
}


#[inline(always)]
pub fn is_valid_email(email: &[u8], config: &Config) -> bool {
    let len = email.len();
    if len < config.email_length.0 as usize {
        return false;
    }

    unsafe {
        let ptr = email.as_ptr();
        let mut at_pos: isize = -1;
        let mut dot_pos: isize = -1;
        let mut has_space = false;

        for i in 0..len {
            let c = *ptr.add(i);
            if c == b'@' {
                if at_pos != -1 {
                    return false;
                }
                at_pos = i as isize;
            } else if c == b'.' {
                dot_pos = i as isize;
            } else if c <= b' ' {
                has_space = true;
            }
        }

        if at_pos == -1 || dot_pos == -1 || has_space {
            return false;
        }

        if at_pos == 0 || dot_pos <= at_pos + 1 || dot_pos == (len - 1) as isize {
            return false;
        }
    }

    true
}

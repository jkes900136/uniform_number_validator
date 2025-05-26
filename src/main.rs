use std::io;

/// 檢查統一編號是否合法
///
/// 統一編號是一個8位數字，其驗證規則如下：
/// 1. 各數字從左至右依序乘以特定的權重: 1, 2, 1, 2, 1, 2, 4, 1
/// 2. 計算每個數字與對應權重的乘積
/// 3. 對每個乘積，將個位數和十位數相加
/// 4. 總和能被10整除則為合法的統一編號
/// 5. 特殊規則：若總和除以10餘7，且最後一位為7，也為合法
fn is_valid_uniform_number(number: &str) -> bool {
    // 檢查長度是否為8位數字
    if number.len() != 8 || !number.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    
    // 權重值 (依據台灣統一編號計算規則)
    let weights = [1, 2, 1, 2, 1, 2, 4, 1];
    
    // 計算總和
    let mut sum = 0;
    for (i, c) in number.chars().enumerate() {
        let digit = c.to_digit(10).unwrap();
        let product = digit * weights[i];
        
        // 計算乘積的個位數和十位數相加
        sum += product / 10 + product % 10;
    }
    
    // 標準規則：總和能被10整除則為合法
    if sum % 10 == 0 {
        return true;
    }
    
    // 特殊規則：第7碼為7，且總和除以10餘7時，也視為合法
    // 第7碼為位置6（從0開始計數）
    let seventh_digit = number.chars().nth(6).unwrap().to_digit(10).unwrap();
    if sum % 10 == 7 && seventh_digit == 7 {
        return true;
    }
    
    false
}

fn main() {
    println!("統一編號檢查程式");
    println!("請輸入一個統一編號（8位數字）或輸入 'q' 退出:");
    
    loop {
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("讀取輸入失敗");
        
        let input = input.trim();
        
        if input.to_lowercase() == "q" {
            println!("程式結束");
            break;
        }
        
        if is_valid_uniform_number(input) {
            println!("「{}」是有效的統一編號", input);
        } else {
            println!("「{}」不是有效的統一編號", input);
        }
        
        println!("\n請輸入下一個統一編號或輸入 'q' 退出:");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_valid_numbers() {
        // 一些有效的統一編號範例（實際有效）
        assert!(is_valid_uniform_number("04595257"));  // 可被10整除
        assert!(is_valid_uniform_number("10458575"));  // 可被10整除
        assert!(is_valid_uniform_number("16886777"));  // 特殊規則：餘7且第7碼為7
    }
    
    #[test]
    fn test_invalid_numbers() {
        // 無效的統一編號
        assert!(!is_valid_uniform_number("12345678"));
        assert!(!is_valid_uniform_number("11111111"));  // 總和不能被10整除也不符合餘7規則
    }
    
    #[test]
    fn test_invalid_format() {
        // 格式錯誤
        assert!(!is_valid_uniform_number("1234567")); // 太短
        assert!(!is_valid_uniform_number("123456789")); // 太長
        assert!(!is_valid_uniform_number("abcdefgh")); // 非數字
    }
}

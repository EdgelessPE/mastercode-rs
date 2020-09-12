fn main() {
  let mut selected_id: Vec<i32> = vec![1, 2, 3, 4, 5, 71, 84, 96, 77, 21, 81, 82, 88];
  let mut init_coding: Vec<i32> = Vec::new();
  let mut nozip_code = String::from("");
  dmsort::sort(&mut selected_id);
  println!("{:#?}", selected_id);
  for i in 0..selected_id.len() {
    if let Some(c) = selected_id.get(i + 1) {
      if let Some(v) = selected_id.get(i) {
        println!("aaa = {}", c - v);
        init_coding.push(c - v);
      }
    }
  }
  println!("{:#?}", init_coding);
  for i in &init_coding {
    if let Ok(v) = code_36::encode_nozip(*i) {
      nozip_code.push_str(&v);
    }
  }
  println!("{}", nozip_code);
  println!("{:#?}", code_36::decode_nozip(&nozip_code.to_string()));
  println!("{:#?}", selected_id);
  let s = String::from("2111111111111111111111111111111111GaE641248");
  println!("{:?}", code_36::zip(&s));
}

mod code_36 {
  use anyhow::*;
  pub fn encode_nozip<'a>(val: i32) -> Result<String> {
    let chars = [
      "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
      "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    let placeholder = ["", "a", "b", "c", "d", "e", "f", "g", "h"];
    let mut result: Vec<&str> = Vec::new();
    let ard = (val - (val % 36)) / 36;
    if ard > 8 {
      return Err(anyhow!("Input Val Overflow!"));
    } else {
      if let Some(v) = placeholder.get((ard) as usize) {
        result.push(v);
      }
      if let Some(v) = chars.get((val - (ard * 36)) as usize) {
        result.push(v)
      }
    }

    return Ok({
      let mut resl = String::new();
      for i in &result {
        resl.push_str(i);
      }
      resl
    });
  }

  pub fn decode_nozip(code: &String) -> Result<Vec<i32>> {
    let chars = [
      '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
      'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let placeholder = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let mut result: Vec<i32> = Vec::new();
    let input_chars: Vec<char> = code.chars().collect();
    let mut i = 0;
    while i <= input_chars.len() - 1 {
      let mut ph_checked = false;
      for c in 0..placeholder.len() {
        if input_chars[i] == placeholder[c] {
          let mut a = 0;
          for c in 0..chars.len() {
            if input_chars[i + 1] == chars[c] {
              a = c;
              break;
            }
          }
          result.push(((c + 1 * 36) + a) as i32);
          ph_checked = true;
          i = i + 2;
          break;
        }
      }
      if ph_checked != true {
        for c in 0..chars.len() {
          if input_chars[i] == chars[c] {
            result.push(c as i32);
            i = i + 1;
            break;
          }
        }
      }
    }
    println!("{:?}", result);
    let mut final_resl: Vec<i32> = Vec::new();
    for i in 0..result.len() {
      final_resl.push({
        let mut resl = 0;
        if let Some(v) = result.get(i) {
          resl = resl + v;
          if i != 0 {
            if let Some(c) = final_resl.get(i - 1) {
              resl = resl + c;
            }
          }
        }
        resl
      } as i32);
    }
    return Ok(final_resl);
  }

  pub fn zip(code: &String) -> Result<String> {
    if code.len() < 3 {
      return Err(anyhow!("Insufficient length,长度为{}", code.len()));
    }
    let mut codeori: Vec<String> = Vec::new();
    for i in code.chars().collect::<Vec<char>>() {
      codeori.push(i.to_string());
    }
    codeori.reverse();
    codeori.pop();
    let mut count: u32 = 0; //z标识符
    let mut cache = "i".to_string();
    let placeholder = ["a", "b", "c", "d", "e", "f", "g", "h"];
    let mut select: String;
    for i in 0..codeori.len() {
      select = codeori.get(i).unwrap().to_string();
      if placeholder.iter().any(|e| e == &select) {
        if select == cache {
          if count != std::u32::MAX {
            count += 1;
          } else {
            codeori[i - 2] = "z".to_string();
            codeori[i - 3] = std::char::from_digit(count, 36)
              .unwrap()
              .to_string()
              .to_uppercase();
            codeori[i - 4] = "y".to_string();
            for j in 5..=count {
              codeori[i - j as usize] = "i".to_string();
            }
            cache = select;
            count = 0;
          }
        } else {
          if count >= 4 {
            codeori[i - 2] = "z".to_string();
            codeori[i - 3] = std::char::from_digit(count, 36)
              .unwrap()
              .to_string()
              .to_uppercase();
            codeori[i - 4] = "y".to_string();
            if count != 4 {
              for j in 5..=count {
                codeori[i - j as usize] = "i".to_string();
              }
            }
          }
          cache = select;
          count = 0;
        }
      }else{
        count=0;
        cache = "i".to_string();
      }
    }
    if count >= 4 {
      codeori[code.len() - 3] = "z".to_string();
      codeori[code.len() - 4] = std::char::from_digit(count, 36)
        .unwrap()
        .to_string()
        .to_uppercase();
      codeori[code.len() - 5] = "y".to_string();
      if count != 4 {
        for j in 5..=count {
          codeori[code.len() - 1 - j as usize] = "i".to_string();
        }
      }
      //count = 0;
    }
    codeori.reverse();
    return Ok({
      let mut resl = String::from("2");
      for i in &codeori {
        if i != "i" {
          resl.push_str(i);
        }
      }
      resl
    });
  }
}

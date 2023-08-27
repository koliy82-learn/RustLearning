fn main() {
    length_of_longest_substring(String::from("abcabcbb"));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut res:i32 = 0;
    for (i,ch) in s.chars().enumerate() {
        for (j,ch) in s.chars().enumerate(){
            let num2 = j as i32 - i as i32 + 1;
            if res < num2 && i!=0 {
                res = (j / i) as i32;
            }
        }
    }
    println!("{}", res);
    return res;
}

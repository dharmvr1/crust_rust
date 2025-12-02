fn main() {
    let mut intput = String::new();
    let mut list = Vec::new();
    loop {
        std::io::stdin()
            .read_line(&mut intput)
            .expect("Failed to read line");
        // println!("You entered: {}", intput.trim());
        if intput.trim().is_empty() {
            break;
        }
        list.push(intput.trim().to_string());
        intput.clear();
    }

    let mut initial_positon = 50;
    let mut password = 0;
    for item in list {
        let direction = item[0..1].to_string();
        let value: i32 = item[1..].trim().parse().unwrap();
        if direction == "L" {
            password += (((100 - initial_positon) % 100 + value) / 100);

            initial_positon -= value;
            initial_positon = ((initial_positon % 100) + 100) % 100;
        } else if direction == "R" {
            password += (initial_positon + value) / 100;
            initial_positon += value;
            initial_positon = initial_positon % 100;
        }
        // if initial_positon == 0 {
        //     password += 1;
        // }
    }

    println!("Password: {}", password);
}

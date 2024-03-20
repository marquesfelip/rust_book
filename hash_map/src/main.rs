fn main() {
    creating_new_hash_map();

    adding_a_key_value_if_key_isnt_present();

    update_value_based_on_old_value();
}

fn creating_new_hash_map() {
    use std::collections::HashMap;

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow "), 50);

    // Accessing Values in a HashMap
    let team_name = String::from("Blue")    ;
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);
}

fn adding_a_key_value_if_key_isnt_present() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

fn update_value_based_on_old_value() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
        println!("{}", count);
    }

    println!("{:?}", map);

}
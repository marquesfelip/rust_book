fn main() {
    updating_vector();
    reading_elements();
    iterating_over_values();
    iterating_over_values_and_make_change();
}

fn updating_vector() {
    // Create new Vector with annotation type
    // let v: Vec<i32> = Vec::new();

    // Create with values the values will define the type
    // let v = vec![1, 2, 3];

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("{}", v[0]);
}

fn reading_elements() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}

fn add_and_holding_a_reference() {
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // The error ocurrs because the memory address will be different after v.push(x)
    // so the reference cannot be possible.
    // v.push(6);

    println!("The first element is: {first}");
}

fn iterating_over_values() {
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{i}");
    }
}

fn iterating_over_values_and_make_change() {
    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
}

fn store_enum_multiple_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
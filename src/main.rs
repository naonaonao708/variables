fn main() { 
    print_labeled_mesurement(5, 'h');
}

fn print_labeled_mesurement(value: i32, unit_label: char) {
    println!("The mesurement is: {}{}", value, unit_label);
}

fn main() {
    let name = "muniz".to_string();
    get_name(name.clone());
    get_name(name.clone());
}


fn get_name(mut name: String) {
    println!("n => {}", name);

    name.remove(3);

    println!("n => {}", name);

}

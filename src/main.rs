use sys_locale::get_locale;

fn main() {
    let locale = get_locale().unwrap();
    println!("{locale}");
}

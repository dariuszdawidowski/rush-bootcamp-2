#[ic_cdk::query]
fn greet(name: String, last_name: String) -> String {
    format!("Hello, {} {}!", name, last_name)
}

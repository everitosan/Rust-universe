use procedural_attribute::log_call;

fn main() {
  println!("Hello world");
  custom("some str");
}

#[log_call(verbose)]
fn custom(a: &str) {
//...
}
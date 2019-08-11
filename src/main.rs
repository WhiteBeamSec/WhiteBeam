// After wrapping this with a boilerplate helpdoc, provide an option to daemonize:
// https://raw.githubusercontent.com/carllerche/tower-web/master/examples/json.rs

pub fn main() {
  let verstr =env!("CARGO_PKG_VERSION");
  println!("WhiteBeam version {}", verstr);
}

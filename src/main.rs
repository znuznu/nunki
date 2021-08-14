mod reader;
mod walker;
// mod error;

fn main() {
    if let Err(e) = walker::walk("samples") {
        eprintln!("{}", e);
    }
}

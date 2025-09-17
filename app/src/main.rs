use gui::run_app;

fn main() {
    match run_app() {
        Ok(_) => (),
        Err(err) => eprintln!("App exited with error:\n{err}"),
    }
}

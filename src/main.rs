mod app;
mod game;
mod ui;

rust_i18n::i18n!("locales", fallback = "en");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    app::run()
}

#[async_std::main]

async fn main() -> Result<(), std::io::Error> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}

slint::include_modules!();

// use exitfailure::ExitFailure;
use reqwest::Url;
// use std::time::Duration;
// use reqwest::ClientBuilder;
// struct CompanyQuote;


// fn api_handler() -> String {
//     let user = "ferris-the-crab";
//     let request_url = format!("https://api.github.com/users/{}", user);
//     println!("{}", request_url);
//     // let url = Url::parse(&*request_url);
//     // let res = reqwest::get(url).await?.json::<api_handler>().await?;
//     return "29191".to_owned();
// }


// #[tokio::main]
fn main() -> Result<(), slint::PlatformError> {
    let body = reqwest::get("https://www.rust-lang.org").await;
    println!("body = {body:?}");
    
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_api_response({move |string| {
            let ui = ui_handle.unwrap();
            let response = "200";
            // let response = api_handler();
            ui.set_preview_output(response.into());
        }
    });

    ui.run()
}

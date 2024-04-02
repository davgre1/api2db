slint::include_modules!();

// fn api_handler(r: &str) -> String  {
//     let user = "ferris-the-crab";
//     let request_url = format!("https://api.github.com/users/{}", user);
//     println!("{}", request_url);
//     // let url = Url::parse(&*request_url);
//     // let res = reqwest::get(url).await?.json::<api_handler>().await?;
//     println!("NAME: {:?}", r);
    
//     return Ok(r.to_string());
// }

use reqwest;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_api_response( {move |string| {
            let ui = ui_handle.unwrap();
            let url_string: &str = string.trim();
            // "https://restcountries.com/v3.1/independent?status=true"
            let mut resp = match reqwest::blocking::get(url_string) {
                // Ok(resp) => serde_json::from_str(resp).unwrap(),
                Ok(resp) => resp.text().unwrap(),
                Err(err) => panic!("Error: {}", err)
            };
            // println!("{}", resp);

            let mut hm = HashMap::new();
            for (key, val) in response.headers().into_iter() {
                hm.insert(key.as_str().to_owned(), val.to_str().ok().unwrap_or("").to_owned());
            }
            let req = Req {status: response.status().as_u16(), url: url_string, body: response.json().ok(), headers: hm};
            println!("{}", serde_json::to_string(&req).unwrap_or("".to_owned()));
            // let json: serde_json::Value = serde_json::from_str(resp);
            // println!("{}", json);

            // let response = api_handler(results);
            ui.set_preview_output(url_string.into());
        }
    });
    ui.run()
}

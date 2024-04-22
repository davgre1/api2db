slint::include_modules!();

// use serde_json::{Value};
use serde_json::{Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};


fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    let ui_handle2 = ui.as_weak();

    // Read API Response and parse headers
    ui.on_api_response( {move |string| {
        let mut resp_map: HashMap<i32, String> = HashMap::new();

        let string = "https://restcountries.com/v3.1/independent?status=true";
        let api_status;
        let ui = ui_handle.unwrap();
        let url_string: &str = string.trim();
        let resp2 = match reqwest::blocking::get(url_string) {
            Ok(resp) => {
                api_status = resp.status().to_string();
                resp.text().unwrap()
            },
            Err(err) => {
                ui.set_error_output("error reading api".into());
                panic!("Error: {}", err);
            }
        };

        // let temp_resp: HashMap<String, Value> = serde_json::from_str(&resp2).unwrap();
        let temp_resp: Vec<Value> = serde_json::from_str(&resp2).unwrap();
        let value: serde_json::Value = serde_json::from_str(&resp2).unwrap();
        let resp_value= temp_resp[0]["altSpellings"].to_string();
        let resp_ln = temp_resp.len();
        println!("{:?}", temp_resp.len()); // 194

        // Finding temp headers
        for key in value[0].as_object().unwrap().keys() {
            println!("{}", key);
        }


        let mut all_people: HashMap<String, Person> = HashMap::new();
        for person in people {
            let key = person.name; 
            let value = Person { age: person.age, registry: person.registry };
            all_people.insert(key, value);
        }
        println!("{:#?}", all_people);

        // let mut tmp = 0;
        // for tr in temp_resp {
        //     println!("{}", tr);
        //     tmp += 1;
        //     resp_map.insert(tmp, tr.to_string());
        // }

        // let mut resp_keys = resp_map.keys();
        // println!("{}", resp_keys.to_string());

        // for rm in resp_map.values() {
        //     println!("{}", rm)
        // }
        
        ui.set_status_code_output(api_status.into());
        ui.set_response_output(resp2.to_string().into());
        ui.set_preview_output(resp_value.to_string().into());
        ui.set_preview_key([0,1,2,3,4,5,6,7,8,9].into());
    }
    });

    // API Response Download to parent directory
    ui.on_api_response_dl( {move |string| {
        let temp_resp: Vec<Value> = serde_json::from_str(&string).unwrap();

        let file = File::create("temp_resp_data.json").unwrap();
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(writer, &temp_resp).expect("Failed writing to file :(");
    }
    });
    ui.run()
}

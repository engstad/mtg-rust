use hyper::Url;
use hyper::client::Request;

use serialize::json;

pub fn test_read() {
    // Creating an outgoing request.
    let loc = "http://mtgjson.com/json/KTK.json";

    let url = match Url::parse(loc.as_slice()) {
        Ok(url) => {
            println!("GET {}...", url)
            url
        },
        Err(e) => panic!("Invalid URL: {}", e)
    };

    let req = match Request::get(url) {
        Ok(req) => req,
        Err(err) => panic!("Failed to connect: {}", err)
    };

    let mut res = req
        .start().unwrap() // failure: Error writing Headers
        .send().unwrap(); // failure: Error reading Response head.

    println!("Response: {}", res.status);
    println!("{}", res.headers);

    let str = res.read_to_string().unwrap();

    let json = json::from_str(str.as_slice());

    match json {
        Ok(doc) => {
            let cards = doc
                .find(&"cards".to_string()).unwrap()
                .as_list().unwrap();

            for card in cards.iter() {
                let name = card.find(&"name".to_string()).unwrap().to_string();
                
                let typ = card.find(&"type".to_string()).unwrap().to_string();

                let typs = card.find(&"types".to_string()).unwrap().as_list().unwrap();

                let image = card.find(&"imageName".to_string()).unwrap().to_string();

                match typs.iter().find(|el| el.to_string().as_slice() == "\"Instant\"")
                {
                    Some(_) => (), None => continue 
                }
                
                print!("{:50s}", name[1..name.len()-1]);
                println!("{}", image[1..image.len()-1]);
                continue;

                println!("{}", typ[1..typ.len()-1]);

                let text = match card.find(&"text".to_string()) {
                    Some(s) => s.to_string(), None => "..".to_string()
                };

                let mut esc = false;
                let mut col = 0i;

                for ch in text[1..text.len()-1].chars() {
                    if esc {
                        match ch {
                            'n' => { col = 0; println!("") },
                            _ => { col += 2; print!("\\{}", ch) }
                        };
                        esc = false;
                    }
                    else {
                        match ch {
                            '\\' => esc = true,
                            ch => { col += 1; print!("{}", ch) }
                        }
                    }
                };
                println!("\n{}", "=".repeat(90));
            }
        },        
        Err(err) => panic!("Error: {}", err)
    }
}

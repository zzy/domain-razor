extern crate reqwest;
extern crate select;

mod email;

use std::fs;

use select::document::Document;
use select::predicate::{ Name};

fn main() -> Result<(), Box<std::error::Error>> {
    
    let file_contents = fs::read_to_string("./domains.txt").expect("读取文件错误");
    let domains = file_contents.split(",");

    for domain in domains {
        let base_url = "http://panda.www.net.cn/cgi-bin/check.cgi?area_domain=";
        let domain_url = base_url.to_string() + domain.trim();

        println!("{}", domain_url);

        let mut res = reqwest::get(&domain_url)?;
        let status = res.status().as_u16();
        let body = res.text().unwrap();
        
        println!("{}", status);
        println!("{:?}", body);
        
        if status == 200 {
            let document = Document::from(body.as_str());
            
            let returncode = document.find(Name("returncode")).next().unwrap().text().parse::<u32>().unwrap();

            if returncode == 200 {
                let key = document.find(Name("key")).next().unwrap().text();
                let original = document.find(Name("original")).next().unwrap().text();

                if original.contains("210") {
                    email::send_email(&key);
                }
            }
            
        }
    }

    Ok(())
}


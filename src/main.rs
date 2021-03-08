use rusoto_core::{Region, HttpClient};
use rusoto_textract::{DetectDocumentTextRequest, Document, Textract, TextractClient};
use std::io::prelude::*;
use std::fs::File;
use std::default::Default;
extern crate image;
extern crate base64;

use rusoto_credential::{EnvironmentProvider};

#[tokio::main]
async fn main() {

    let creds = EnvironmentProvider::default();

    let mut file = File::open("node_js.png").unwrap();
    
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    
    let byt = bytes::Bytes::from(buf);
    let doc = Document {
        bytes: Some(byt),
        s3_object: None,
    };
    let client = TextractClient::new_with(
        HttpClient::new().unwrap(), creds, Region::ApSoutheast1);
    let document = DetectDocumentTextRequest {
        document: doc,
    };

    let response = client.detect_document_text(document).await;
    match response {
        Ok(res) => {
            match res.blocks {
                Some(dta) => {
                    for item in dta {
                        if item.block_type.unwrap() == "LINE" {
                            println!("{}", item.text.unwrap());
                        }
                    }
                }
                None => println!("")
            }
        }
        Err(e) => println!("{}", e.to_string())
    }
}

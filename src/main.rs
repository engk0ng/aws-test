use rusoto_core::{Region};
use rusoto_textract::{DetectDocumentTextRequest, Document, Textract, TextractClient};
use std::io;
use std::io::prelude::*;
use std::fs::File;
extern crate image;
extern crate base64;

#[tokio::main]
async fn main() {
    let mut file = File::open("js.png").unwrap();
    
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap();
    
    let byt = bytes::Bytes::from(buf);
    let doc = Document {
        bytes: Some(byt),
        s3_object: None,
    };
    let client = TextractClient::new(Region::ApSoutheast1);
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

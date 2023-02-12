
use hyper::body::Buf;
use hyper::{header,Body,Client,Request};
use  hyper_tls::HttpsConnector;
use serde_derive::{Deserialize,Serialize};
// use std::fmt::Display;
use std::env;
use::std::io::{stdin,stdout,Write};
#[derive(Deserialize, Debug)]
struct Choice {
    text: String,
    // ... other fields
}
#[derive(Deserialize,Debug)]
struct OAIResponse{
    choices: Vec<Choice>,
   id:Option<String>,
   object:Option<String>,
   created:Option<u64>,
   model:Option<String>,
   
}
#[derive(Serialize,Debug)]
struct OAIRequest{ 
prompt:String,
max_tokens:u32
}

#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error + Send + Sync>>{
    // let  api_key=env::var("API_KEY").expect("api key should be there");
let https=HttpsConnector::new();
let client=Client::builder().build(https);
let uri="https://api.openai.com/v1/engines/text-davinci-001/completions";
let preamble="answer the following and please give a short answer";
let auth_header_val=format!("Bearer{}",apikey);
// morning--cli tool
println!("{esc}c",esc =27 as char);
loop{
    print!(">");
    stdout().flush().unwrap();
    let mut user_text=String::new();
    stdin().read_line(&mut user_text)
    .expect("failed to read line");
    
    println!("");

    let oai_request=OAIRequest{
        prompt:format!("{} {}",preamble,user_text),
        max_tokens:100
    };
    let body=Body::from(serde_json::to_vec(&oai_request)?);
    let req=Request::post(uri)
    .header(header::CONTENT_TYPE,"application/json")
    .header("Authorization", &auth_header_val)
    .body(body)
    .unwrap();
    let res=client.request(req).await?;
    let body=hyper::body::aggregate(res).await?;
    let  json:OAIResponse=serde_json::from_reader(body.reader())?;
  
println!("");

// name
// 

}
}
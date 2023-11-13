use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize,Clone,Debug)]
pub struct Message {
    pub role : String,
    pub content : String,
}

#[derive(Serialize, Deserialize,Clone)]
pub struct ChatCompletion {
    pub model : String,
    pub messages : Vec<Message>,
    pub temperature : f32,
}

#[derive( Deserialize)] 
pub struct APIMessage {
    pub content : String,
}

#[derive( Deserialize)] 
pub struct  APIChoice {
    pub message: APIMessage
}

#[derive( Deserialize)]
pub struct APIResponse {
    pub choices : Vec<APIChoice>,
}
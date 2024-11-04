use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
pub struct CreatePost{
    pub title:String,
    pub content:String
}

#[derive(Deserialize,Serialize,Debug)]
pub struct UpdatePost {
    pub title:Option<String>,
    pub content:Option<String>
}


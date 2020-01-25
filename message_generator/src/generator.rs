use crate::message::MsgSpec;

pub trait Generator {
    fn generate_messages(messages: &Vec<MsgSpec>, UID: u32) -> Vec<(String, String)>; //return Vec<(filename, txt)>    TODO: improve lisibility (make a struct ?)
}

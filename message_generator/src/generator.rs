use crate::message::MsgSpec;

pub trait Generator {
    fn generate_code(messages: Vec<MsgSpec>) -> Vec<(String, String)>; //return Vec<(filename, txt)>    TODO: improve lisibility (make a struct ?)
}

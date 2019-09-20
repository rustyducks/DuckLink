use crate::message::{MsgSpec, Type};
use toml::value::{Table, Value};

// TODO refaire ça proprement

pub fn parse_toml(contents: &str) -> Result<Vec<MsgSpec>, std::io::Error> {
    let value = contents.parse::<Value>()?;

    let up_messages = &value["up"];
    let down_messages = &value["down"];

    let mut messages = vec![];

    if let Value::Table(t) = up_messages {
        messages = parse_message_class(t)?;
    }

    if let Value::Table(t) = down_messages {
        let mut down_messages = parse_message_class(t)?;
        messages.append(&mut down_messages);
    }

    Ok(messages)
}

fn parse_message_class(t: &Table) -> Result<Vec<MsgSpec>, std::io::Error> {
    let mut messages = vec![];

    for (msg_name, m) in t {
        if let Value::Table(msg_table) = m {
            let mut msg = MsgSpec::new(msg_name.to_string());
            for (name, typ) in msg_table {
                match Type::from_toml(typ) {
                    Ok(t) => {
                        msg.fields.push((name.to_string(), t));
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            messages.push(msg);
        }
    }

    Ok(messages)
}

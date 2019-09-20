use crate::message::{Field, MsgSpec, Type};
use toml::value::{Table, Value};

// TODO refaire ça proprement

pub fn parse_toml(contents: &str) -> Result<Vec<MsgSpec>, Vec<String>> {
    let value = contents.parse::<Value>().map_err(|_e| vec!["IoError at TOML parsing!".to_string()])?;

    let mut messages = vec![];
    let msg_classes = ["up", "down"];

    for msg_class in msg_classes.iter() {
        if let Some(up_messages) = value.get(msg_class) {
            if let Value::Table(t) = up_messages {
                let mut msgs = parse_message_class(t)?;
                messages.append(&mut msgs);
            }
        }
    }

    Ok(messages)
}

fn parse_message_class(t: &Table) -> Result<Vec<MsgSpec>, Vec<String>> {
    let msg_errs_tuples = t.iter()
    .filter_map(|(msg_name, m)| get_messages(msg_name, m))
    .collect::<Vec<_>>();

    // let mut msgs = vec![];
    // let mut errs = vec![];
    // for (m, e) in msg_errs_tuples {
    //     msgs.push(m);
    //     errs.push(e);
    // }

    let (msgs, errs) = msg_errs_tuples.into_iter()
        .fold((Vec::<MsgSpec>::new(), Vec::new()), |(mut msgs, mut errs), (m, e)| {
            msgs.push(m);
            errs.push(e);
            (msgs, errs)
        });

    let errs = errs.into_iter().flatten().collect::<Vec<_>>();

    if errs.is_empty() {
        Ok(msgs)
    } else {
        Err(errs)
    }
    
    
}



fn get_messages(msg_name:&str, msg_table: &Value) -> Option<(MsgSpec, Vec<String>)> {
    if let Value::Table(msg_table) = msg_table {
        let (fields, errs): (Vec<_>, Vec<_>) = msg_table
        .iter()
        .map(|(name, typ)| {
            match Type::from_toml(typ) {
                Ok(pty) => Ok(Field{name:name.to_string(), t:pty}),
                Err(e) => Err(format!("{}.{}: {}", msg_name, name, e))//Err((msg_name, name, e))
            }
        })
        .partition(Result::is_ok);
        let fields: Vec<_> = fields.into_iter().map(Result::unwrap).collect();
        let errs: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();


        Some((MsgSpec{name: msg_name.to_string(), fields}, errs))
    } else {
        None
    }
}
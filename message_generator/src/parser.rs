use crate::message::{Field, MsgSpec, Type};
use inflector::Inflector;
use toml::value::{Table, Value};

// TODO refaire ça proprement

pub fn parse_toml(contents: &str) -> Result<Vec<MsgSpec>, Vec<String>> {
    let value = contents
        .parse::<Value>()
        .map_err(|_e| vec!["IoError at TOML parsing!".to_string()])?;

    let mut messages = vec![];

    match value {
        Value::Table(t_root) => {
            for (class, msgs) in t_root {
                if let Value::Table(msgs) = msgs {
                    let mut msgs = parse_message_class(&class.to_class_case(), &msgs)?;
                    messages.append(&mut msgs);
                }
            }
        }
        _ => println!("ERROR ! Not a table !"),
    }

    for (i, msg) in messages.iter_mut().enumerate() {
        msg.id = i;
    }

    Ok(messages)
}

fn parse_message_class(class: &str, t: &Table) -> Result<Vec<MsgSpec>, Vec<String>> {
    let msg_errs_tuples = t
        .iter()
        .filter_map(|(msg_name, m)| get_messages(class, &msg_name.to_class_case(), m))
        .collect::<Vec<_>>();

    let (msgs, errs) = msg_errs_tuples.into_iter().fold(
        (Vec::<MsgSpec>::new(), Vec::new()),
        |(mut msgs, mut errs), (m, e)| {
            msgs.push(m);
            errs.push(e);
            (msgs, errs)
        },
    );

    let errs = errs.into_iter().flatten().collect::<Vec<_>>();

    if errs.is_empty() {
        Ok(msgs)
    } else {
        Err(errs)
    }
}

fn get_messages(class: &str, msg_name: &str, msg_table: &Value) -> Option<(MsgSpec, Vec<String>)> {
    let mut name = class.to_string();
    name.push_str(msg_name);
    if let Value::Table(msg_table) = msg_table {
        let (fields, errs): (Vec<_>, Vec<_>) = msg_table
            .iter()
            .map(|(name, typ)| match Type::from_toml(typ) {
                Ok(pty) => Ok(Field {
                    name: name.to_string(),
                    t: pty,
                }),
                Err(e) => Err(format!("{}.{}: {}", msg_name, name, e)),
            })
            .partition(Result::is_ok);
        let fields: Vec<_> = fields.into_iter().map(Result::unwrap).collect();
        let errs: Vec<_> = errs.into_iter().map(Result::unwrap_err).collect();

        Some((
            MsgSpec {
                name,
                id: 0,
                fields,
            },
            errs,
        ))
    } else {
        None
    }
}

use crate::generator::Generator;
use crate::message::{MsgSpec, Type};

pub struct PythonGenerator;

impl PythonGenerator {
    const HEADER: &'static str = "from duckmsg import DuckMsg, clamp";

    fn declare_class(msg: &MsgSpec) -> String {
        let msg_id = format!("\t\tself.id = {}", msg.id);

        let declarations = if msg.fields.len() == 0 {
            "\t\tpass".to_string()
        } else {
            msg.fields
                .iter()
                .map(|field| {
                    format!(
                        "\t\t{}",
                        PythonGenerator::init_variable(field.name.as_ref(), &field.t)
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        };

        let getters = msg
            .fields
            .iter()
            .map(|field| PythonGenerator::make_get_set(&field.name, &field.t))
            .collect::<Vec<String>>()
            .join("\n\n");

        let code = format!(
            "class {}(DuckMsg):\n\tdef __init__(self):\n{}\n{}\n\n{}",
            msg.name, msg_id, declarations, getters
        );

        code
    }

    fn init_variable(name: &str, ty: &Type) -> String {
        match ty {
            Type::I8(_b) | Type::I16(_b) | Type::I32(_b) => format!("self._{} = 0", name),
            Type::U8(_b) | Type::U16(_b) | Type::U32(_b) => format!("self._{} = 0", name),
            Type::F32(_b) => format!("self._{} = 0.0", name),
            Type::CHARS(_size) => format!("self._{} = b''", name),
        }
    }

    fn make_get_set(name: &str, ty: &Type) -> String {
        let getter = format!(
            "\t@property\n\tdef {name}(self):\n\t\treturn self._{name}",
            name = name
        );

        let setter = match ty {
            Type::CHARS(_) => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}={name}", name=name),
            Type::I8(b)|Type::I16(b)|Type::I32(b)|
            Type::U8(b)|Type::U16(b)|Type::U32(b)
                => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min}, {name}, {max})", name=name, min=b.min, max=b.max),
            Type::F32(b) => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min}, {name}, {max})", name=name, min=b.min, max=b.max),
        };
        format!("{}\n\n{}", getter, setter)
    }
}

impl Generator for PythonGenerator {
    fn generate_messages(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let classes = messages
            .iter()
            .map(|msg| PythonGenerator::declare_class(msg))
            .collect::<Vec<String>>()
            .join("\n\n");

        let code = format!("{}\n\n{}\n", PythonGenerator::HEADER, classes);

        vec![("messages.py".to_string(), code)]
    }
}

use crate::generator::Generator;
use crate::message::{Type, MsgSpec};

pub struct PythonGenerator;

impl PythonGenerator {

    const HEADER :&'static str = "import duckmsg";

    fn declare_class(msg: &MsgSpec) -> String {
        let declarations =
            if msg.fields.len() == 0 {
                "\t\tpass".to_string()
            } else {
                msg.fields.iter()
                .map(
                    |(field_name, t)| format!("\t\t{}",  PythonGenerator::init_variable(t, field_name.as_ref()))
                    )
                .collect::<Vec<String>>().join("\n")
            };

        let code = format!("class {}(DuckMsg):\n\tdef __init__(self):\n{}", msg.name, declarations);
        
        code
    }


    fn init_variable(ty: &Type, name: &str) -> String {
        match ty {
            Type::I8 | Type::I16 | Type::I32 => format!("self.{} = 0", name),
            Type::U8 | Type::U16 | Type::U32 => format!("self.{} = 0", name),
            Type::F32 => format!("self.{} = 0.0", name),
            Type::CHARS(_size) => format!("self.{} = b''", name)
        }
    }
}

impl Generator for PythonGenerator {

    fn generate_code(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let classes = messages.iter().map(|msg| PythonGenerator::declare_class(msg)).collect::<Vec<String>>().join("\n\n");
        
        let code = format!("{}\n\n{}\n", PythonGenerator::HEADER, classes);

        vec![("messages.py".to_string(), code)]
    }
}

use crate::generator::Generator;
use crate::message::{Type, MsgSpec};

pub struct CGenerator;

impl CGenerator {

    const HEADER :&'static str = "#ifndef MESSAGES_H\n#define MESSAGES_H\n\n#include <stdint.h>";
    const FOOTER :&'static str = "#endif    // MESSAGES_H";

    fn declare_struct(msg: &MsgSpec) -> String {
        let vars = msg.fields.iter()
                    .map(
                        |(field_name, t)| format!("  {}",  CGenerator::init_variable(t, field_name.as_ref()))
                        )
                    .collect::<Vec<String>>().join("\n");

        let code = format!("struct {} {{\n{}\n}}", msg.name, vars);

        code
    }



    fn init_variable(ty: &Type, name: &str) -> String {
        match ty {
            Type::I8 => format!("int8_t {};", name),
            Type::I16 => format!("int16_t {};", name),
            Type::I32 => format!("int32_t {};", name),
            Type::U8 => format!("uint8_t {}", name),
            Type::U16 => format!("uint16_t {};", name),
            Type::U32 => format!("uint32_t {};", name),
            Type::F32 => format!("float32_t {};", name),
            Type::CHARS(size) => format!("char {}[{}];", name, size)
        }
    }
}

impl Generator for CGenerator {

    fn generate_code(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let declarations = messages.iter().map(|msg| CGenerator::declare_struct(msg)).collect::<Vec<String>>().join("\n\n");
        
        let code = format!("{}\n\n{}\n\n{}", CGenerator::HEADER, declarations, CGenerator::FOOTER);

        vec![("messages.h".to_string(), code)]
    }
}
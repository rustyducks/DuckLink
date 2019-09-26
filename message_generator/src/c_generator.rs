use crate::generator::Generator;
use crate::message::{MsgSpec, Type};

pub struct CGenerator;

impl CGenerator {
    const HEADER: &'static str = "#ifndef MESSAGES_H\n#define MESSAGES_H\n\n#include <stdint.h>";
    const FOOTER: &'static str = "#endif    // MESSAGES_H";

    fn declare_struct(msg: &MsgSpec) -> String {
        let vars = msg
            .fields
            .iter()
            .map(|field| {
                format!(
                    "  {}",
                    CGenerator::init_variable(field.name.as_ref(), &field.t)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let code = format!("struct {} {{\n{}\n}}", msg.name, vars);

        code
    }

    fn init_variable(name: &str, ty: &Type) -> String {
        match ty {
            Type::I8(_b) => format!("int8_t {};", name),
            Type::I16(_b) => format!("int16_t {};", name),
            Type::I32(_b) => format!("int32_t {};", name),
            Type::U8(_b) => format!("uint8_t {}", name),
            Type::U16(_b) => format!("uint16_t {};", name),
            Type::U32(_b) => format!("uint32_t {};", name),
            Type::F32(_b) => format!("float32_t {};", name),
            Type::CHARS(size) => format!("char {}[{}];", name, size),
        }
    }
}

impl Generator for CGenerator {
    fn generate_messages(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let declarations = messages
            .iter()
            .map(|msg| CGenerator::declare_struct(msg))
            .collect::<Vec<String>>()
            .join("\n\n");

        let code = format!(
            "{}\n\n{}\n\n{}",
            CGenerator::HEADER,
            declarations,
            CGenerator::FOOTER
        );

        vec![("messages.h".to_string(), code)]
    }
}

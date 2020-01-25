use crate::generator::Generator;
use crate::message::{MsgSpec, Type};

pub struct CPPGenerator;

impl CPPGenerator {
    const HEADER_H: &'static str = "#ifndef MESSAGES_H\n#define MESSAGES_H\n\n#include <stdint.h>\n#include <string.h>\n#include \"Duckmsg.h\"";
    const FOOTER_H: &'static str = "#endif    // MESSAGES_H";

    const HEADER_CPP: &'static str = "#include \"messages.h\"";
    const FOOTER_CPP: &'static str = "";

    fn declare_class(msg: &MsgSpec) -> String {
        let vars = msg
            .fields
            .iter()
            .map(|field| {
                format!(
                    "  {}",
                    CPPGenerator::declare_variable(field.name.as_ref(), &field.t)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

        let getsets = msg
            .fields
            .iter()
            .map(|field| CPPGenerator::make_get_set(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n\n");

        let msg_size: usize = msg.get_buffer_size(); // 2 start bytes, 1 byte for the ID, 1 for the length, ..., 2 for the checksum

        let code = format!(
            "class {name}: public DuckMsg {{\npublic:\n  \
             static const size_t SIZE = {size};\n  \
             static const uint8_t ID = {id};\n\n  \
             {name}();\n  \
             {name}(uint8_t *buffer);\n\n  \
             void to_bytes(uint8_t *buffer);\n\n\
             {getsets}\n\n\
             private:\n\
             {vars}\n}};",
            name = msg.name,
            size = msg_size,
            id = msg.id,
            getsets = getsets,
            vars = vars
        );

        code
    }

    fn deserialise_var(name: &str, ty: &Type) -> String {
        format!(
            "  memcpy(&_{name}, buffer+offset, {size});\n  \
             offset += {size};",
            name = name,
            size = ty.get_size()
        )
    }

    fn serialise_var(name: &str, ty: &Type) -> String {
        format!(
            "  memcpy(buffer+offset, &_{name}, {size});\n  \
             offset += {size};",
            name = name,
            size = ty.get_size()
        )
    }

    fn to_bytes(msg: &MsgSpec) -> String {
        let serialisations = msg
            .fields
            .iter()
            .map(|field| CPPGenerator::serialise_var(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n");

        let code = format!(
            "void {name}::to_bytes(uint8_t *buffer) {{\n  \
             int offset = 0;\n  \
             buffer[offset++] = 0xFF;\n  \
             buffer[offset++] = 0xFF;\n  \
             buffer[offset++] = ID;\n  \
             buffer[offset++] = {lenght};\n\
             {serialisations}\n  \
             int16_t checksum = compute_cheksum(buffer+2, {lenght});\n  \
             buffer[offset++] = checksum & 0XFF;\n  \
             buffer[offset++] = (checksum>>8) & 0XFF;\n\
             }}",
            name = msg.name,
            serialisations = serialisations,
            lenght = msg.get_payload_size() + 2
        );

        code
    }

    fn get_type(ty: &Type) -> &str {
        match ty {
            Type::I8(_b) => "int8_t",
            Type::I16(_b) => "int16_t",
            Type::I32(_b) => "int32_t",
            Type::U8(_b) => "uint8_t",
            Type::U16(_b) => "uint16_t",
            Type::U32(_b) => "uint32_t",
            Type::F32(_b) => "float",
            Type::CHARS(_size) => "char*",
        }
    }

    fn declare_variable(name: &str, ty: &Type) -> String {
        match ty {
            Type::I8(_b) => format!("int8_t _{};", name),
            Type::I16(_b) => format!("int16_t _{};", name),
            Type::I32(_b) => format!("int32_t _{};", name),
            Type::U8(_b) => format!("uint8_t _{};", name),
            Type::U16(_b) => format!("uint16_t _{};", name),
            Type::U32(_b) => format!("uint32_t _{};", name),
            Type::F32(_b) => format!("float _{};", name),
            Type::CHARS(size) => format!("char _{}[{}];", name, size),
        }
    }

    fn make_get_set(name: &str, ty: &Type) -> String {
        let setter = match ty {
            Type::CHARS(size) => format!(
                "  void set_{name}({t} {name}) {{\n    strncpy(_{name}, {name}, {size});\n  }}",
                name = name,
                t = CPPGenerator::get_type(ty),
                size = size
            ),
            Type::I8(b)
            | Type::I16(b)
            | Type::I32(b)
            | Type::U8(b)
            | Type::U16(b)
            | Type::U32(b) => format!(
                "  void set_{name}({t} {name}){{ _{name} = clamp({min}, {name}, {max}); }}",
                name = name,
                t = CPPGenerator::get_type(ty),
                min = b.min,
                max = b.max
            ),
            Type::F32(b) => format!(
                "  void set_{name}({t} {name}){{ _{name} = clamp({min:.2}, {name}, {max:.2}); }}",
                name = name,
                t = CPPGenerator::get_type(ty),
                min = b.min,
                max = b.max
            ),
        };

        let getter = format!(
            "  {t} get_{name}(){{ return _{name}; }}",
            t = CPPGenerator::get_type(ty),
            name = name
        );

        format!("{}\n{}", getter, setter)
    }

    fn init_variable(name: &str, ty: &Type) -> String {
        match ty {
            Type::I8(_b) => format!("  _{} = 0;", name),
            Type::I16(_b) => format!("  _{} = 0;", name),
            Type::I32(_b) => format!("  _{} = 0;", name),
            Type::U8(_b) => format!("  _{} = 0;", name),
            Type::U16(_b) => format!("  _{} = 0;", name),
            Type::U32(_b) => format!("  _{} = 0;", name),
            Type::F32(_b) => format!("  _{} = 0;", name),
            Type::CHARS(_size) => format!("  _{}[0] = \'\\0\';", name),
        }
    }

    fn constructor(msg: &MsgSpec) -> String {
        let vars = msg
            .fields
            .iter()
            .map(|field| CPPGenerator::init_variable(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n");

        let code = format!(
            "{name}::{name}() {{\n{vars}\n}}",
            name = msg.name,
            vars = vars
        );

        code
    }

    fn constructor_from_bytes(msg: &MsgSpec) -> String {
        let deserialisations = msg
        .fields
        .iter()
        .map(|field| CPPGenerator::deserialise_var(field.name.as_ref(), &field.t))
        .collect::<Vec<String>>()
        .join("\n");

        let code = format!(
            "{name}::{name}(uint8_t *buffer) {{\n  \
                int offset = 0;\n  \
                {deser}\n}}",
            name = msg.name,
            deser = deserialisations
        );

        code
    }

    fn make_msg(messages: &Vec<MsgSpec>) -> String {
        let ifs = messages.iter()
                    .map(|msg| {
                        format!("\tif id=={id} {{\n\t\t\
                                    return {name}();\n\t\
                                }}", id=msg.id, name=msg.name)
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

        format!("DuckMsg make_msg(uint8_t id) {{\n{}\n}}", ifs)
    }
}

impl Generator for CPPGenerator {
    fn generate_messages(messages: &Vec<MsgSpec>, UID: u32) -> Vec<(String, String)> {
        let declarations = messages
            .iter()
            .map(|msg| CPPGenerator::declare_class(msg))
            .collect::<Vec<String>>()
            .join("\n\n\n");

        let header = format!(
            "{}\n\n\
            DuckMsg make_msg(uint8_t id);\n\n\
            {}\n\n{}",
            CPPGenerator::HEADER_H,
            declarations,
            CPPGenerator::FOOTER_H
        );

        let serialisations = messages
            .iter()
            .map(|msg| {
                format!(
                    "{}\n\n{}\n\n{}",
                    CPPGenerator::constructor(msg),
                    CPPGenerator::constructor_from_bytes(msg),
                    CPPGenerator::to_bytes(msg),
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n\n");
        
        let make_msg = CPPGenerator::make_msg(messages);

        let source = format!(
            "{}\n\n{}\n\n{}\n\n{}",
            CPPGenerator::HEADER_CPP,
            make_msg,
            serialisations,
            CPPGenerator::FOOTER_CPP
        );

        vec![
            ("messages.h".to_string(), header),
            ("messages.cpp".to_string(), source),
        ]
    }
}

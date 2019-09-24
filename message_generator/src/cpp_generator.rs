use crate::generator::Generator;
use crate::message::{MsgSpec, Type};

pub struct CPPGenerator;

impl CPPGenerator {
    const HEADER: &'static str = "#ifndef MESSAGES_H\n#define MESSAGES_H\n\n#include <stdint.h>\n#include <string.h>\n#include \"Duckmsg.h\"";
    const FOOTER: &'static str = "#endif    // MESSAGES_H";

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

        let msg_id = format!("  uint8_t _id;");

        let constructor = CPPGenerator::constructor(msg);

        let getsets = msg
            .fields
            .iter()
            .map(|field| CPPGenerator::make_get_set(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n\n");

        let get_id = "  uint8_t get_id(){ return _id; }";

        let code = format!("class {name}: public DuckMsg {{\npublic:\n{constructor}\n\n{getid}\n\n{getsets}\n\nprivate:\n{id}\n{vars}\n}};", name=msg.name, constructor=constructor, getid=get_id, getsets=getsets, id=msg_id, vars=vars);

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
        // let setter = match ty {
        //     Type::CHARS(_) => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}={name}", name=name),
        //     Type::I8(b)|Type::I16(b)|Type::I32(b)|
        //     Type::U8(b)|Type::U16(b)|Type::U32(b)
        //         => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min}, {name}, {max})", name=name, min=b.min, max=b.max),
        //         => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min}, {name}, {max})", name=name, min=b.min, max=b.max),
        // };

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
                "  void set_{name}({t} {name}){{ _{name} = clamp({min}, {name}, {max}); }}",
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
            Type::I8(_b) => format!("    _{} = 0;", name),
            Type::I16(_b) => format!("    _{} = 0;", name),
            Type::I32(_b) => format!("    _{} = 0;", name),
            Type::U8(_b) => format!("    _{} = 0;", name),
            Type::U16(_b) => format!("    _{} = 0;", name),
            Type::U32(_b) => format!("    _{} = 0;", name),
            Type::F32(_b) => format!("    _{} = 0;", name),
            Type::CHARS(_size) => format!("    _{}[0] = \'\\0\';", name),
        }
    }

    fn constructor(msg: &MsgSpec) -> String {
        let init_id = format!("    _id = {};", msg.id);

        let vars = msg
            .fields
            .iter()
            .map(|field| CPPGenerator::init_variable(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n");

        let code = format!(
            "  {name}() {{\n{id}\n{vars}\n  }}",
            name = msg.name,
            id = init_id,
            vars = vars
        );

        code
    }
}

impl Generator for CPPGenerator {
    fn generate_code(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let declarations = messages
            .iter()
            .map(|msg| CPPGenerator::declare_class(msg))
            .collect::<Vec<String>>()
            .join("\n\n\n");

        let code = format!(
            "{}\n\n{}\n\n{}",
            CPPGenerator::HEADER,
            declarations,
            CPPGenerator::FOOTER
        );

        vec![("messages.h".to_string(), code)]
    }
}

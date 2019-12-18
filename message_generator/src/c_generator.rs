use crate::generator::Generator;
use crate::message::{MsgSpec, Type};
extern crate inflector;
use inflector::Inflector;

pub struct CGenerator;

impl CGenerator {
    const HEADER_H: &'static str = "#ifndef MESSAGES_H\n#define MESSAGES_H\n\n#include <stdint.h>\n#include <string.h>";
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
                    CGenerator::declare_variable(field.name.as_ref(), &field.t)
                )
            })
            .collect::<Vec<String>>()
            .join("\n");

// let getsets = msg
//     .fields
//     .iter()
//     .map(|field| CGenerator::make_get_set(field.name.as_ref(), &field.t))
//     .collect::<Vec<String>>()
//     .join("\n\n");

        let size = msg.get_size() + 6;  // 0XFF, 0XFF (2), id (1), len (1), ..., chk (2)

        let code = format!(
            "#define SIZE_{name} {size}\n\
            #define  ID_{name} {id}\n\n\
            struct {name}{{\n\
             {vars}\n}};\n\n\
             void {sname}_from_bytes(union Message_t* msg_u, uint8_t *buffer);\n\
             void {sname}_to_bytes(struct {name}* msg, uint8_t *buffer);\n\n\
             ",
            size=size,
            id=msg.id,
            name = msg.name,
            sname = msg.name.to_snake_case(),
            vars = vars
        );

        code
    }

    fn deserialise_var(name: &str, ty: &Type) -> String {
        format!(
            "  memcpy(&msg->{name}, buffer+offset, {size});\n  \
             offset += {size};",
            name = name,
            size = ty.get_size()
        )
    }

    fn serialise_var(name: &str, ty: &Type) -> String {
        format!(
            "  memcpy(buffer+offset, &msg->{name}, {size});\n  \
             offset += {size};",
            name = name,
            size = ty.get_size()
        )
    }

    fn to_bytes(msg: &MsgSpec) -> String {
        let serialisations = msg
            .fields
            .iter()
            .map(|field| CGenerator::serialise_var(field.name.as_ref(), &field.t))
            .collect::<Vec<String>>()
            .join("\n");

        let code = format!(
            "void {sname}_to_bytes(struct {name}* msg, uint8_t *buffer) {{\n  \
             int offset = 0;\n  \
             buffer[offset++] = 0xFF;\n  \
             buffer[offset++] = 0xFF;\n  \
             buffer[offset++] = ID_{name};\n  \
             buffer[offset++] = {lenght};\n\
             {serialisations}\n  \
             int16_t checksum = compute_cheksum(buffer+2, {lenght});\n  \
             buffer[offset++] = checksum & 0XFF;\n  \
             buffer[offset++] = (checksum>>8) & 0XFF;\n\
             }}",
            sname = msg.name.to_snake_case(),
            name = msg.name,
            serialisations = serialisations,
            lenght = msg.get_size() + 2
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
            Type::I8(_b) => format!("int8_t {};", name),
            Type::I16(_b) => format!("int16_t {};", name),
            Type::I32(_b) => format!("int32_t {};", name),
            Type::U8(_b) => format!("uint8_t {};", name),
            Type::U16(_b) => format!("uint16_t {};", name),
            Type::U32(_b) => format!("uint32_t {};", name),
            Type::F32(_b) => format!("float {};", name),
            Type::CHARS(size) => format!("char {}[{}];", name, size),
        }
    }

    fn make_get_set(name: &str, ty: &Type) -> String {
        let setter = match ty {
            Type::CHARS(size) => format!(
                "  void set_{name}({t} {name}) {{\n    strncpy({name}, {name}, {size});\n  }}",
                name = name,
                t = CGenerator::get_type(ty),
                size = size
            ),
            Type::I8(b)
            | Type::I16(b)
            | Type::I32(b)
            | Type::U8(b)
            | Type::U16(b)
            | Type::U32(b) => format!(
                "  void set_{name}({t} {name}){{ {name} = clamp({min}, {name}, {max}); }}",
                name = name,
                t = CGenerator::get_type(ty),
                min = b.min,
                max = b.max
            ),
            Type::F32(b) => format!(
                "  void set_{name}({t} {name}){{ {name} = clamp({min:.2}, {name}, {max:.2}); }}",
                name = name,
                t = CGenerator::get_type(ty),
                min = b.min,
                max = b.max
            ),
        };

        let getter = format!(
            "  {t} get_{name}(){{ return {name}; }}",
            t = CGenerator::get_type(ty),
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

    // fn constructor(msg: &MsgSpec) -> String {
    //     let vars = msg
    //         .fields
    //         .iter()
    //         .map(|field| CGenerator::init_variable(field.name.as_ref(), &field.t))
    //         .collect::<Vec<String>>()
    //         .join("\n");

    //     let code = format!(
    //         "{name}::{name}() {{\n{vars}\n}}",
    //         name = msg.name,
    //         vars = vars
    //     );

    //     code
    // }

    fn constructor_from_bytes(msg: &MsgSpec) -> String {
        let deserialisations = msg
        .fields
        .iter()
        .map(|field| CGenerator::deserialise_var(field.name.as_ref(), &field.t))
        .collect::<Vec<String>>()
        .join("\n");

        let code = format!(
            "void {sname}_from_bytes(union Message_t* msg_u, uint8_t *buffer) {{\n  \
                struct {name}* msg = (struct {name}*)msg_u;\n  \
                int offset = 0;\n\
                {deser}\n}}",
            sname = msg.name.to_snake_case(),
            name = msg.name,
            deser = deserialisations
        );

        code
    }

    fn make_msg(messages: &Vec<MsgSpec>) -> String {
        let ifs = messages.iter()
                    .map(|msg| {
                        format!("\tif(id=={id}) {{\n\t\t\
                                {sname}_from_bytes(msg_u, buffer);\n\t\
                                }}", id=msg.id, sname=msg.name.to_snake_case())
                    })
                    .collect::<Vec<String>>()
                    .join("\n");

        format!("void msg_from_bytes(union Message_t* msg_u, uint8_t* buffer, uint8_t id) {{\n\
                {}\t\
                \n}}", ifs)
    }
}

impl Generator for CGenerator {
    fn generate_messages(messages: Vec<MsgSpec>) -> Vec<(String, String)> {
        let declarations = messages
            .iter()
            .map(|msg| CGenerator::declare_class(msg))
            .collect::<Vec<String>>()
            .join("\n\n\n");
        
        let union_fields = messages
            .iter()
            .map(|msg| format!("  struct {} {};", msg.name, msg.name.to_snake_case()))
            .collect::<Vec<String>>()
            .join("\n");
        let union_t = format!("union Message_t {{\n{}\n}};", union_fields);

        let header = format!(
            "{}\n\n\
            union Message_t;\n\n\
            uint16_t compute_cheksum(uint8_t *buffer, int len);\n\n\
            {}\n\n\
            {}\n\
            void make_msg(union Message_t* msg, uint8_t id);\n\n\
            void msg_from_bytes(union Message_t* msg_u, uint8_t* buffer, uint8_t id);\n\n\
            {}",
            CGenerator::HEADER_H,
            declarations,
            union_t,
            CGenerator::FOOTER_H
        );


        let serialisations = messages
            .iter()
            .map(|msg| {
                format!(
                    "{tb}\n\n{fb}",
                    fb=CGenerator::constructor_from_bytes(msg),
                    tb=CGenerator::to_bytes(msg),
                )
            })
            .collect::<Vec<String>>()
            .join("\n\n\n");
        
        let make_msg = CGenerator::make_msg(&messages);

        let check = "uint16_t compute_cheksum(uint8_t *buffer, int len) {\n\t\
                        (void)buffer;\n\t\
                        (void)len;\n\t\
                        return 10;\n\
                    }";

        let source = format!(
            "{}\n\n{}\n\n{}\n\n{}\n\n{}",
            CGenerator::HEADER_CPP,
            check,
            make_msg,
            serialisations,
            CGenerator::FOOTER_CPP
        );

        vec![
            ("messages.h".to_string(), header),
            ("messages.c".to_string(), source),
        ]
    }
}

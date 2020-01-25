use crate::generator::Generator;
use crate::message::{MsgSpec, Type};

pub struct PythonGenerator;

impl PythonGenerator {
    const HEADER: &'static str = "from duckmsg import DuckMsg, clamp\n\
                                  import bitstring";

    fn declare_class(msg: &MsgSpec) -> String {
        let msg_id = format!("\tID = {}", msg.id);
        let msg_size = format!("\tSIZE = {}", msg.get_buffer_size());

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

        let serialize = PythonGenerator::serialize(msg);
        let deserialize = PythonGenerator::deserialize(msg);

        let repr = PythonGenerator::repr(msg);

        let code = format!(
            "class {name}(DuckMsg):\n{id}\n{size}\n\tdef __init__(self):\n{dec}\n\n{serialize}\n\n{deserialize}\n\n{repr}\n\n{gets}",
            name=msg.name, id=msg_id, size=msg_size, dec=declarations, serialize=serialize, deserialize=deserialize, repr=repr, gets=getters
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
                => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min:.1}, {name}, {max:.1})", name=name, min=b.min, max=b.max),
            Type::F32(b) => format!("\t@{name}.setter\n\tdef {name}(self, {name}):\n\t\tself._{name}=clamp({min:.1}, {name}, {max:.1})", name=name, min=b.min, max=b.max),
        };
        format!("{}\n\n{}", getter, setter)
    }

    fn repr(msg: &MsgSpec) -> String {
        let fields = msg.fields
            .iter()
            .map(|field| {
                format!("'{name} : {{}}'.format(self._{name})", name=field.name)
            })
            .collect::<Vec<String>>()
            .join(", ");
        
        format!("\tdef __repr__(self):\n\t\t\
                    return '\\n'.join([{}])",
            fields)
    }

    fn serialize(msg: &MsgSpec) -> String {
        
        let fields = msg.fields
            .iter()
            .map(|field| {
                format!("self.{}", field.name)
            })
            .collect::<Vec<String>>()
            .join(", ");

        let bit_format = msg.fields
            .iter()
            .map(|field| {
                let bitype = match field.t {
                    Type::I8(_) => "intle:8".to_string(),
                    Type::I16(_) => "intle:16".to_string(),
                    Type::I32(_) => "intle:32".to_string(),
                    Type::U8(_)=> "uintle:8".to_string(),
                    Type::U16(_) => "uintle:16".to_string(),
                    Type::U32(_) => "uintle:32".to_string(),
                    Type::F32(_) => "floatle:32".to_string(),
                    Type::CHARS(s) => format!("bytes:{}", s),
                };
                format!("{}", bitype)
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("\tdef serialize(self):\n\t\t\
                        return bitstring.pack('uintle:8, uintle:8, {bit_format}', self.ID, self.SIZE-4, {fields})",
                    fields=fields, bit_format=bit_format)
    }

    fn deserialize(msg: &MsgSpec) -> String {
        
        let fields = msg.fields
            .iter()
            .map(|field| {
                format!("self.{}", field.name)
            })
            .collect::<Vec<String>>()
            .join(", ");

        let bit_format = msg.fields
            .iter()
            .map(|field| {
                let bitype = match field.t {
                    Type::I8(_) => "intle:8".to_string(),
                    Type::I16(_) => "intle:16".to_string(),
                    Type::I32(_) => "intle:32".to_string(),
                    Type::U8(_)=> "uintle:8".to_string(),
                    Type::U16(_) => "uintle:16".to_string(),
                    Type::U32(_) => "uintle:32".to_string(),
                    Type::F32(_) => "floatle:32".to_string(),
                    Type::CHARS(s) => format!("bytes:{}", s),
                };
                format!("{}", bitype)
            })
            .collect::<Vec<String>>()
            .join(", ");

        format!("\tdef deserialize(self, bytes):\n\t\t\
                    s = bitstring.BitStream(bytes)\n\t\t\
                    {fields} = s.unpack('{bit_format}')",
                    fields=fields, bit_format=bit_format)
    }

    fn message_dict(messages: &Vec<MsgSpec>) -> String{
        let body = messages
            .iter()
            .map(|msg| {
                format!("\t{id} : {name},", id=msg.id, name=msg.name)
            })
            .collect::<Vec<String>>()
            .join("\n");
        
            format!("MESSAGES = {{\n{}\n}}", body)
    }
}

impl Generator for PythonGenerator {
    fn generate_messages(messages: &Vec<MsgSpec>, uid: u32) -> Vec<(String, String)> {
        let classes = messages
            .iter()
            .map(|msg| PythonGenerator::declare_class(msg))
            .collect::<Vec<String>>()
            .join("\n\n");
        
        let dict = PythonGenerator::message_dict(messages);

        let uid_code = format!("UID = {}", uid);

        let code = format!("{}\n\n{}\n\n{}\n\n{}\n", PythonGenerator::HEADER, uid_code, classes, dict);

        vec![("messages.py".to_string(), code)]
    }
}

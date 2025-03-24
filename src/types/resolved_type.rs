use std::{collections::HashMap, fmt, rc::Rc};


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResolvedType {
    Integer, Double, Boolean,
    Function(FunctionType),
    Struct(StructType),
    Pointer(PointerType),
    Empty,
}

impl fmt::Display for ResolvedType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResolvedType::Integer => write!(f, "{{\"type\": \"Integer\"}}"),
            ResolvedType::Double => write!(f, "{{\"type\": \"Double\"}}"),
            ResolvedType::Boolean => write!(f, "{{\"type\": \"Boolean\"}}"),
            ResolvedType::Function(func) => write!(f, "{{\"type\": \"Function\", \"function\": {}}}", func),
            ResolvedType::Struct(struct_type) => write!(f, "{{\"type\": \"Struct\", \"struct\": {}}}", struct_type),
            ResolvedType::Pointer(ptr) => write!(f, "{{\"type\": \"Pointer\", \"pointer\": {}}}", ptr),
            ResolvedType::Empty => write!(f, "{{\"type\": \"Empty\"}}")
        }
    }
}

impl ResolvedType {
    pub fn n_bytes(&self) -> usize {
        match self {
            ResolvedType::Integer => 8,
            ResolvedType::Double => 8,
            ResolvedType::Boolean => 8,
            ResolvedType::Function(_) => 8,
            ResolvedType::Struct(struct_type) => struct_type.n_bytes(),
            ResolvedType::Pointer(_) => 8,
            ResolvedType::Empty => 0
        }
    }

    pub fn is_numeric(&self) -> bool {
        match self {
            ResolvedType::Integer => true,
            ResolvedType::Double => true,
            _ => false
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct PointerType {
    pub pointee: Rc<ResolvedType>
}

impl fmt::Display for PointerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"pointee\": {}}}", self.pointee)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct StructType {
    size: usize, /* in bytes */
    member_offsets: Rc<HashMap<String, usize>>, /* location, in bytes, for each member in the struct */
    member_types: Rc<HashMap<String, ResolvedType>>,
}

impl fmt::Display for StructType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"size\": {}, \"member_offsets\":{{", self.size)?;

        for (i, (name, ty)) in self.member_types.iter().enumerate() {
            write!(f, "\"{}\": {}", name, ty)?;

            if i < self.member_types.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}, \"member_types\":{{")?;

        for (i, (name, offset)) in self.member_offsets.iter().enumerate() {
            write!(f, "\"{}\": {}", name, offset)?;

            if i < self.member_offsets.len() - 1 {
                write!(f, ", ")?;
            }
        }

        write!(f, "}}}}")
    }
}

impl StructType {
    pub fn new(members: Vec<(String, ResolvedType)>) -> StructType {
        let mut member_offsets = HashMap::new();
        let mut member_types = HashMap::new();
        let mut offset = 0;

        for (name, t) in members {
            member_offsets.insert(name.clone(), offset);
            offset += t.n_bytes();

            member_types.insert(name.clone(), t);
        }

        StructType {
            size: offset,
            member_offsets: Rc::new(member_offsets),
            member_types: Rc::new(member_types),
        }
    }

    pub fn get_member_type(&self, member: &str) -> &ResolvedType {
        self.member_types.get(member).unwrap()
    }

    pub fn get_member_offset(&self, member: &str) -> usize {
        *self.member_offsets.get(member).unwrap()
    }

    pub fn n_bytes(&self) -> usize {
        self.size
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct FunctionType {
    pub arg_types: Rc<Vec<ResolvedType>>,
    pub ret_type: Rc<ResolvedType>
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{\"arg_types\": [")?;

        for arg in self.arg_types.iter() {
            write!(f, "{},", arg)?;
        }

        write!(f, "], \"ret_type\": {}}}", self.ret_type)
    }
}

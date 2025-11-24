use std::{collections::HashMap, fmt, rc::Rc};


#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ResolvedType {
    Integer, Double, Boolean,
    Struct(StructType),
}

impl ResolvedType {
    pub fn n_bytes(&self) -> usize {
        match self {
            ResolvedType::Integer => 8,
            ResolvedType::Double => 8,
            ResolvedType::Boolean => 8,
            ResolvedType::Struct(struct_type) => struct_type.n_bytes(),
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
pub struct StructType {
    size: usize, /* in bytes */
    member_offsets: Rc<HashMap<String, usize>>, /* location, in bytes, for each member in the struct */
    member_types: Rc<HashMap<String, ResolvedType>>,
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
use crate::init::Variables;


impl Variables{
    pub fn match_var(&self,to_match_with:&Variables) -> bool{

        match (self,to_match_with){
            (Variables::I8(_), Variables::I8(_)) => return true,
            (Variables::I16(_), Variables::I16(_)) => return true,
            (Variables::I32(_), Variables::I32(_)) => return true,
            (Variables::I64(_), Variables::I64(_)) => return true,
            (Variables::U8(_), Variables::U8(_)) => return true,
            (Variables::U16(_), Variables::U16(_)) => return true,
            (Variables::U32(_), Variables::U32(_)) => return true,
            (Variables::U64(_), Variables::U64(_)) => return true,
            (Variables::F32(_), Variables::F32(_)) => return true,
            (Variables::F64(_), Variables::F64(_)) => return true,
            (Variables::Bool(_), Variables::Bool(_)) => return true,
            (Variables::Str(_), Variables::Str(_)) => return true,
            (Variables::AsIs(_), Variables::AsIs(_)) => return true,
            _ => false,
        };

        true
    }
    pub fn get_value(&self) -> String{
        match self{
            Variables::I8(val) => return val.to_string(),
            Variables::I16(val) => return val.to_string(),
            Variables::I32(val) => return val.to_string(),
            Variables::I64(val) => return val.to_string(),
            Variables::U8(val) => return val.to_string(),
            Variables::U16(val) => return val.to_string(),
            Variables::U32(val) => return val.to_string(),
            Variables::U64(val) => return val.to_string(),
            Variables::F32(val) => return val.to_string(),
            Variables::F64(val) => return val.to_string(),
            Variables::Bool(val) => return val.to_string(),
            Variables::Str(val) => return val.to_string(),
            Variables::AsIs(val) => return val.to_string(),
        }
    }
}
use crate::value::{Value, ValueData};
use std::{collections::HashMap, fmt};

pub type ObjectData = HashMap<String, Property>;

#[derive(Trace, Finalize, Clone, Debug)]
pub struct Property {
    pub value: Value,
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self.value.borrow())
    }
}

impl Property {
    pub fn new(value: Value) -> Self {
        Property { value: value }
    }
}
impl ValueData {
    pub fn set_field(&self, field: &String, val: &Value) {
        match self {
            ValueData::Object(obj) => {
                obj.borrow_mut()
                    .insert(field.clone(), Property::new(val.clone()));
            }
            _ => (),
        }
    }

    pub fn get_prop(&self, field: &String) -> Option<Property> {
        let obj: ObjectData = match *self {
            ValueData::Object(ref obj) => obj.clone().into_inner(),
            _ => return None,
        };

        obj.get(field).map(|f| f.clone())
    }
}

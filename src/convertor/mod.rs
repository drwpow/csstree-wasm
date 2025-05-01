use std::collections::HashMap;

pub struct Convertor<F>
where
    F: Fn(&mut HashMap<String, serde_json::Value>, &str),
{
    walk: F,
}

impl<F> Convertor<F>
where
    F: Fn(&mut HashMap<String, serde_json::Value>, &str),
{
    pub fn new(walk: F) -> Self {
        Convertor { walk }
    }

    pub fn from_plain_object(
        &self,
        ast: &mut HashMap<String, serde_json::Value>,
    ) -> &mut HashMap<String, serde_json::Value> {
        (self.walk)(ast, "enter");
        if let Some(children) = ast.get_mut("children") {
            if !children.is_array() {
                let array = children
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .clone()
                    .into_iter()
                    .collect();
                *children = serde_json::Value::Array(array);
            }
        }
        ast
    }

    pub fn to_plain_object(
        &self,
        ast: &mut HashMap<String, serde_json::Value>,
    ) -> &mut HashMap<String, serde_json::Value> {
        (self.walk)(ast, "leave");
        if let Some(children) = ast.get_mut("children") {
            if children.is_array() {
                let array = children
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .clone()
                    .into_iter()
                    .collect();
                *children = serde_json::Value::Array(array);
            }
        }
        ast
    }
}

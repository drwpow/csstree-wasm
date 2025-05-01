use std::collections::HashMap;

pub struct Walker {
    types: HashMap<String, TypeConfig>,
    iterators_natural: HashMap<String, TypeIterator>,
    iterators_reverse: HashMap<String, TypeIterator>,
    break_walk: Symbol,
    skip_node: Symbol,
}

pub struct TypeConfig {
    context: Option<String>,
    fields: Vec<FieldConfig>,
}

pub struct FieldConfig {
    name: String,
    field_type: FieldType,
    nullable: bool,
}

pub enum FieldType {
    Node,
    List,
}

pub struct Symbol(String);

impl Walker {
    pub fn new(config: Config) -> Self {
        let types = Walker::get_types_from_config(&config);
        let mut iterators_natural = HashMap::new();
        let mut iterators_reverse = HashMap::new();
        let break_walk = Symbol("break-walk".to_string());
        let skip_node = Symbol("skip-node".to_string());

        for (name, type_config) in &types {
            iterators_natural.insert(
                name.clone(),
                Walker::create_type_iterator(type_config, false),
            );
            iterators_reverse.insert(
                name.clone(),
                Walker::create_type_iterator(type_config, true),
            );
        }

        Walker {
            types,
            iterators_natural,
            iterators_reverse,
            break_walk,
            skip_node,
        }
    }

    fn get_types_from_config(config: &Config) -> HashMap<String, TypeConfig> {
        let mut types = HashMap::new();

        for (name, node_type) in &config.node {
            if let Some(structure) = &node_type.structure {
                types.insert(
                    name.clone(),
                    Walker::get_walkers_from_structure(name, structure),
                );
            } else {
                panic!(
                    "Missed `structure` field in `{}` node type definition",
                    name
                );
            }
        }

        types
    }

    fn get_walkers_from_structure(
        name: &str,
        structure: &HashMap<String, Vec<FieldType>>,
    ) -> TypeConfig {
        let mut fields = Vec::new();

        for (key, field_types) in structure {
            let mut field_config = FieldConfig {
                name: key.clone(),
                field_type: FieldType::Node, // Default type
                nullable: false,
            };

            for field_type in field_types {
                match field_type {
                    FieldType::Node => field_config.field_type = FieldType::Node,
                    FieldType::List => field_config.field_type = FieldType::List,
                }
            }

            fields.push(field_config);
        }

        TypeConfig {
            context: None, // Add context if needed
            fields,
        }
    }

    fn create_type_iterator(type_config: &TypeConfig, reverse: bool) -> TypeIterator {
        let mut fields = type_config.fields.clone();
        if reverse {
            fields.reverse();
        }

        TypeIterator { fields }
    }

    pub fn walk<F>(&self, root: &Node, options: Option<WalkOptions>, callback: F)
    where
        F: Fn(&Node, Option<&Item>, Option<&List>) -> Option<Symbol>,
    {
        let mut enter = noop;
        let mut leave = noop;
        let mut iterators = &self.iterators_natural;

        if let Some(options) = options {
            if options.reverse {
                iterators = &self.iterators_reverse;
            }

            if let Some(visit) = options.visit {
                if let Some(iterator) = iterators.get(&visit) {
                    // Use specific iterator
                } else {
                    panic!("Bad value `{}` for `visit` option", visit);
                }
            }
        }

        // Walk logic here
    }
}

pub struct Config {
    pub node: HashMap<String, NodeType>,
}

pub struct NodeType {
    pub structure: Option<HashMap<String, Vec<FieldType>>>,
}

pub struct WalkOptions {
    pub reverse: bool,
    pub visit: Option<String>,
}

pub struct Node {
    pub node_type: String,
    // Add other fields as needed
}

pub struct Item;
pub struct List;

pub struct TypeIterator {
    fields: Vec<FieldConfig>,
}

fn noop() {}

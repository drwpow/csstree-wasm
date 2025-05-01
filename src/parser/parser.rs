use std::collections::HashMap;

#[derive(Debug)]
pub struct Lexer {
    css_wide_keywords: Vec<String>,
    generic: bool,
    units: HashMap<String, Vec<String>>,
    atrules: HashMap<String, Atrule>,
    properties: HashMap<String, Property>,
    types: HashMap<String, Type>,
    structure: HashMap<String, Structure>,
}

#[derive(Debug)]
pub struct Atrule {
    name: String,
    prelude: Option<SyntaxDescriptor>,
    descriptors: Option<HashMap<String, SyntaxDescriptor>>,
}

#[derive(Debug)]
pub struct Property {
    name: String,
    syntax: Option<SyntaxDescriptor>,
}

#[derive(Debug)]
pub struct Type {
    name: String,
    syntax: Option<SyntaxDescriptor>,
}

#[derive(Debug)]
pub struct SyntaxDescriptor {
    syntax: Option<String>,
    match_graph: Option<String>, // Placeholder for match graph
}

#[derive(Debug)]
pub struct Structure {
    // Placeholder for structure validation
}

impl Lexer {
    pub fn new(config: Option<Config>) -> Self {
        let mut lexer = Lexer {
            css_wide_keywords: vec![
                "inherit".to_string(),
                "initial".to_string(),
                "unset".to_string(),
            ],
            generic: false,
            units: HashMap::new(),
            atrules: HashMap::new(),
            properties: HashMap::new(),
            types: HashMap::new(),
            structure: HashMap::new(),
        };

        if let Some(config) = config {
            if let Some(css_wide_keywords) = config.css_wide_keywords {
                lexer.css_wide_keywords = css_wide_keywords;
            }

            if let Some(units) = config.units {
                lexer.units = units;
            }

            if let Some(types) = config.types {
                for (name, syntax) in types {
                    lexer.add_type(name, syntax);
                }
            }

            if config.generic {
                lexer.generic = true;
                // Add generic types logic here
            }

            if let Some(atrules) = config.atrules {
                for (name, atrule) in atrules {
                    lexer.add_atrule(name, atrule);
                }
            }

            if let Some(properties) = config.properties {
                for (name, property) in properties {
                    lexer.add_property(name, property);
                }
            }
        }

        lexer
    }

    pub fn add_atrule(&mut self, name: String, atrule: Atrule) {
        self.atrules.insert(name, atrule);
    }

    pub fn add_property(&mut self, name: String, property: Property) {
        self.properties.insert(name, property);
    }

    pub fn add_type(&mut self, name: String, syntax: SyntaxDescriptor) {
        self.types.insert(
            name,
            Type {
                name,
                syntax: Some(syntax),
            },
        );
    }

    pub fn validate(&self) -> Option<ValidationResult> {
        let mut errors = Vec::new();
        let mut broken_types = HashMap::new();
        let mut broken_properties = HashMap::new();

        for (key, ty) in &self.types {
            self.validate_syntax(key, &mut broken_types, ty.syntax.as_ref(), &mut errors);
        }

        for (key, property) in &self.properties {
            self.validate_syntax(
                key,
                &mut broken_properties,
                property.syntax.as_ref(),
                &mut errors,
            );
        }

        let broken_types_array: Vec<_> = broken_types
            .iter()
            .filter(|(_, &broken)| broken)
            .map(|(name, _)| name.clone())
            .collect();

        let broken_properties_array: Vec<_> = broken_properties
            .iter()
            .filter(|(_, &broken)| broken)
            .map(|(name, _)| name.clone())
            .collect();

        if !broken_types_array.is_empty() || !broken_properties_array.is_empty() {
            Some(ValidationResult {
                errors,
                types: broken_types_array,
                properties: broken_properties_array,
            })
        } else {
            None
        }
    }

    fn validate_syntax(
        &self,
        name: &str,
        broken_map: &mut HashMap<String, bool>,
        syntax: Option<&SyntaxDescriptor>,
        errors: &mut Vec<String>,
    ) {
        if broken_map.contains_key(name) {
            return;
        }

        broken_map.insert(name.to_string(), false);

        if let Some(syntax) = syntax {
            // Walk through the syntax and validate references
            // Placeholder for syntax validation logic
        }
    }

    pub fn dump(&self, syntax_as_ast: bool, pretty: bool) -> DumpResult {
        DumpResult {
            generic: self.generic,
            css_wide_keywords: self.css_wide_keywords.clone(),
            units: self.units.clone(),
            types: self.dump_map_syntax(&self.types, syntax_as_ast, pretty),
            properties: self.dump_map_syntax(&self.properties, syntax_as_ast, pretty),
            atrules: self.dump_atrule_map_syntax(&self.atrules, syntax_as_ast, pretty),
        }
    }

    fn dump_map_syntax<T>(
        &self,
        map: &HashMap<String, T>,
        syntax_as_ast: bool,
        pretty: bool,
    ) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for (name, _value) in map {
            // Placeholder for dumping logic
            result.insert(name.clone(), "syntax".to_string());
        }

        result
    }

    fn dump_atrule_map_syntax(
        &self,
        map: &HashMap<String, Atrule>,
        syntax_as_ast: bool,
        pretty: bool,
    ) -> HashMap<String, String> {
        let mut result = HashMap::new();

        for (name, atrule) in map {
            // Placeholder for dumping atrule logic
            result.insert(name.clone(), "atrule".to_string());
        }

        result
    }
}

#[derive(Debug)]
pub struct Config {
    pub css_wide_keywords: Option<Vec<String>>,
    pub units: Option<HashMap<String, Vec<String>>>,
    pub types: Option<HashMap<String, SyntaxDescriptor>>,
    pub generic: bool,
    pub atrules: Option<HashMap<String, Atrule>>,
    pub properties: Option<HashMap<String, Property>>,
}

#[derive(Debug)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub types: Vec<String>,
    pub properties: Vec<String>,
}

#[derive(Debug)]
pub struct DumpResult {
    pub generic: bool,
    pub css_wide_keywords: Vec<String>,
    pub units: HashMap<String, Vec<String>>,
    pub types: HashMap<String, String>,
    pub properties: HashMap<String, String>,
    pub atrules: HashMap<String, String>,
}

fn main() {
    let config = Config {
        css_wide_keywords: Some(vec![
            "inherit".to_string(),
            "initial".to_string(),
            "unset".to_string(),
        ]),
        units: None,
        types: None,
        generic: false,
        atrules: None,
        properties: None,
    };

    let lexer = Lexer::new(Some(config));
    println!("{:#?}", lexer);
}

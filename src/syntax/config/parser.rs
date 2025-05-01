use crate::tokenizer::TokenType;

pub struct ParserConfig {
    parseContext: ParseContext,
    features: ParserConfigFeatures,
}

pub struct ParseContext {
    default: TokenType,
    stylesheet: TokenType,
    atrule: TokenType,
    mediaQueryList: TokenType,
    mediaQuery: TokenType,
    rule: TokenType,
    selectorList: TokenType,
    selector: TokenType,
    declarationList: TokenType,
    declaration: TokenType,
    value: TokenType,
}

pub struct ParserConfigFeatures {
    supports: ParserConfigSupports,
    container: ParserConfigContainer,
}

pub struct ParserConfigSupports {
    selector: fn(selector: Selector) -> bool,
}

pub struct ParserConfigContainer {
    style: fn(style: Style) -> bool,
}

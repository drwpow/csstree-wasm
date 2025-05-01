import fs from "node:fs";
import * as csstree from "../pkg/csstree_wasm.js";
import * as tokenizer from "../pkg/tokenizer.js";
import parse from "../pkg/parser.js";
import parseSelector from "../pkg/selector-parser.js";
import walk from "../pkg/walker.js";
import generate from "../pkg/generator.js";
import convertor from "../pkg/convertor.js";
import * as lexer from "../pkg/lexer.js";
import * as definitionSyntax from "../pkg/definition-syntax.js";
import data from "../pkg/definition-syntax-data.js";
import dataPatch from "../pkg/definition-syntax-data-patch.js";
import * as utils from "../pkg/utils.js";
import { describe, expect, it } from "vitest";

const stringifyWithNoInfo = (ast) =>
  JSON.stringify(ast, (key, value) => (key !== "loc" ? value : undefined), 4);
const fixtureFilename = "./fixtures/stringify.css";
const css = fs.readFileSync(fixtureFilename, "utf-8");
const expectedAst = JSON.parse(
  fs.readFileSync(fixtureFilename.replace(".css", ".ast"), "utf8"),
);

describe("exports / entry points", () => {
  it("tokenizer", () => {
    expect(Object.keys(tokenizer).sort()).toEqual([
      "AtKeyword",
      "BadString",
      "BadUrl",
      "CDC",
      "CDO",
      "Colon",
      "Comma",
      "Comment",
      "Delim",
      "DigitCategory",
      "Dimension",
      "EOF",
      "EofCategory",
      "Function",
      "Hash",
      "Ident",
      "LeftCurlyBracket",
      "LeftParenthesis",
      "LeftSquareBracket",
      "NameStartCategory",
      "NonPrintableCategory",
      "Number",
      "OffsetToLocation",
      "Percentage",
      "RightCurlyBracket",
      "RightParenthesis",
      "RightSquareBracket",
      "Semicolon",
      "String",
      "TokenStream",
      "Url",
      "WhiteSpace",
      "WhiteSpaceCategory",
      "charCodeCategory",
      "cmpChar",
      "cmpStr",
      "consumeBadUrlRemnants",
      "consumeEscaped",
      "consumeName",
      "consumeNumber",
      "decodeEscaped",
      "findDecimalNumberEnd",
      "findWhiteSpaceEnd",
      "findWhiteSpaceStart",
      "getNewlineLength",
      "isBOM",
      "isDigit",
      "isHexDigit",
      "isIdentifierStart",
      "isLetter",
      "isLowercaseLetter",
      "isName",
      "isNameStart",
      "isNewline",
      "isNonAscii",
      "isNonPrintable",
      "isNumberStart",
      "isUppercaseLetter",
      "isValidEscape",
      "isWhiteSpace",
      "tokenNames",
      "tokenTypes",
      "tokenize",
    ]);
  });

  it("parser", () => {
    const ast = parse(css);
    expect(stringifyWithNoInfo(ast)).toEqual(stringifyWithNoInfo(expectedAst));
  });

  describe("selector-parser", () => {
    const selectors = [];

    // collect all the selectors
    csstree.walk(expectedAst, function (node) {
      if (node.type === "SelectorList" || node.type === "Selector") {
        selectors.push({
          css: csstree.generate(node),
          context: node.type === "SelectorList" ? "selectorList" : "selector",
          expected: node,
        });

        return csstree.walk.skip;
      }
    });

    for (const test of selectors) {
      it(test.css, () => {
        const ast = parseSelector(test.css, { context: test.context });
        expect(stringifyWithNoInfo(ast)).toEqual(
          stringifyWithNoInfo(test.expected),
        );
      });
    }
  });

  it("generator", () => {
    expect(generate(expectedAst)).toEqual(csstree.generate(expectedAst));
  });

  it("walker", () => {
    const actualTypes = [];
    const expectedTypes = [];

    walk(expectedAst, (node) => actualTypes.push(node.type));
    csstree.walk(expectedAst, (node) => expectedTypes.push(node.type));

    expect(actualTypes).toBe(expectedTypes);
  });

  it("convertor", () => {
    const ast = parse(css);
    const findFirstAtrule = (ast) =>
      csstree.walk.find(ast, (node) => node.type === "Atrule");

    expect(stringifyWithNoInfo(ast)).toBe(true);
    expect(findFirstAtrule(ast).prelude.children instanceof utils.List).toBe(
      true,
    );

    convertor.toPlainObject(ast);

    expect(Array.isArray(ast.children)).toBe(true);
    expect(Array.isArray(findFirstAtrule(ast).prelude.children)).toBe(true);

    convertor.fromPlainObject(ast);

    expect(ast.children instanceof utils.List).toBe(true);
    expect(findFirstAtrule(ast).prelude.children instanceof utils.List).toBe(
      true,
    );

    expect(Object.keys(convertor).sort()).toEqual([
      "fromPlainObject",
      "toPlainObject",
    ]);
  });

  it("lexer", () => {
    expect(lexer.Lexer).toBeTypeOf("function");
    expect(Object.keys(lexer).sort()).toEqual(["Lexer"]);
  });

  it("definitionSyntax", () => {
    expect(Object.keys(definitionSyntax).sort()).toEqual([
      "SyntaxError",
      "generate",
      "parse",
      "walk",
    ]);
  });

  it("data", () => {
    expect(Object.keys(data).sort()).toEqual([
      "atrules",
      "properties",
      "types",
    ]);
  });

  it("data-patch", () => {
    expect(Object.keys(dataPatch).sort()).toEqual([
      "atrules",
      "properties",
      "types",
    ]);
  });

  it("utils", () => {
    expect(Object.keys(utils).sort()).toEqual([
      "List",
      "clone",
      "ident",
      "isCustomProperty",
      "keyword",
      "property",
      "string",
      "url",
      "vendorPrefix",
    ]);
    expect(utils.string.encode("foo")).toBe('"foo"');
    expect(utils.keyword("-webkit-foo").basename).toBe("foo");
  });
});

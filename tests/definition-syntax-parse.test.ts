import { lexer, definitionSyntax } from "../pkg/csstree_wasm.js";
import { describe, expect, it } from "vitest";

const { parse } = definitionSyntax;

describe("definitionSyntax.parse()", () => {
  describe("combinator precedence", () => {
    const combinators = [" ", "&&", "||", "|"]; // higher goes first
    const print = (node) => {
      return node.type === "Group"
        ? `(${node.terms.map(print).join(node.combinator)})`
        : node.name;
    };

    for (const hi of combinators) {
      for (const lo of combinators.slice(combinators.indexOf(hi) + 1)) {
        describe(`"${hi}" vs. "${lo}"`, () => {
          const hilo = `a${hi}b${lo}c`;
          it(hilo, () => {
            const ast = parse(hilo);
            expect(print(ast)).toBe(`((a${hi}b)${lo}c)`);
          });

          const lohi = `a${lo}b${hi}c`;
          it(lohi, () => {
            const ast = parse(lohi);
            expect(print(ast)).toBe(`(a${lo}(b${hi}c))`);
          });
        });
      }
    }
  });

  describe("bad syntax", () => {
    it("expected a quote", () =>
      expect(() => parse("'x")).toThrowError(
        /^SyntaxError: Expect an apostrophe\n/,
      ));

    describe("expected a number", () => {
      const tests = ["<x>{}", "<x>{ }", "<x>{,2}", "<x>{ ,2}"];

      for (const test of tests) {
        it(test, () =>
          expect(() => parse(test)).toThrowError(
            /^SyntaxError: Expect a number\n/,
          ),
        );
      }
    });

    describe("missed keyword", () => {
      const tests = ["<>", "<''>"];

      for (const test of tests) {
        it(test, () =>
          expect(() => parse(test)).toThrowError(
            /^SyntaxError: Expect a keyword\n/,
          ),
        );
      }
    });

    describe("unexpected combinator", () => {
      const tests = ["<x>&&", "&&<x>", "<x>&&||"];

      for (const test of tests) {
        it(test, () =>
          expect(() => parse(test)).toThrowError(
            /^SyntaxError: Unexpected combinator\n/,
          ),
        );
      }
    });

    describe("unexpected input", () => {
      const tests = ["#", "?", "+", "*", "!", "[]]", "{1}"];

      for (const test of tests) {
        it(test, () =>
          expect(() => parse(test)).toThrowError(
            /^SyntaxError: Unexpected input\n/,
          ),
        );
      }
    });

    describe("bad syntax", () => {
      const tests = ["a&b", "<a", "[a"];

      for (const test of tests) {
        it(test, () =>
          expect(() => parse(test)).toThrowError(/^SyntaxError: Expect `.`\n/),
        );
      }
    });
  });

  // FIXME: in fact this test checks lexer's dictionaries, since lexer uses
  // parse under the hood when first access to definition's "syntax" property;
  // Problems:
  // - test has side effect, because access to syntax trigger parsing and cache the result
  // - some other tests can init syntax
  // - dictionary can move or store actual ast and don't use parsing et all
  // - etc.
  describe("parse", () => {
    for (const section of ["properties", "types"]) {
      for (const [name, definition] of Object.entries(lexer[section])) {
        if (definition.serializable) {
          it(`${section}/${name}`, () => {
            expect(definition.syntax.type).toBe("Group");
          });
        }
      }
    }

    for (const [name, definition] of Object.entries(lexer.atrules)) {
      describe(`atrules/${name}`, () => {
        if (definition.prelude !== null) {
          it("prelude", () =>
            expect(definition.prelude.syntax.type).toBe("Group"));
        }

        if (definition.descriptors) {
          describe("definitions", () => {
            for (const name in definition.descriptors) {
              it(name, () =>
                expect(definition.descriptors[name].syntax.type).toBe("Group"),
              );
            }
          });
        }
      });
    }
  });
});

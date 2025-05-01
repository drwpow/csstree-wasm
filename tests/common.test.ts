import fs from "node:fs";
import path from "node:path";
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import {
  parse,
  walk,
  fork,
  version,
  tokenTypes,
  generate,
} from "../pkg/csstree_wasm.js";

const fixtureFilename = "./fixtures/stringify.css";
const fixture = normalize(fs.readFileSync(fixtureFilename, "utf-8"));
const types = Object.keys(parse.config.node)
  .sort()
  .filter((type) => type !== "DeclarationList"); // DeclarationList doesn't appear in StyleSheet

function normalize(str) {
  return str.replace(/\n|\r\n?|\f/g, "\n");
}

describe("Common", () => {
  it("should expose version", () => {
    expect(version).toBe(
      JSON.parse(fs.readFileSync("./package.json", "utf8")).version,
    );
  });

  it("JSON.stringify()", () => {
    const ast = parse(fixture, {
      filename: path.basename(fixtureFilename),
      positions: true,
    });

    // fs.writeFileSync(fixtureFilename.replace(/\.css/, '.ast'), JSON.stringify(ast, null, 4) + '\n', 'utf-8');

    expect(JSON.stringify(ast, null, 4)).toBe(
      normalize(fs.readFileSync("./fixtures/stringify.ast", "utf-8").trim()),
    );
  });

  it("test CSS should contain all node types", () => {
    const foundTypes = new Set();
    const ast = parse(fixture);

    walk(ast, (node) => foundTypes.add(node.type));

    expect([...foundTypes].sort()).toEqual(
      types
        .sort()
        .filter((type) => type !== "WhiteSpace"), // FIXME: temporary filter white space
    );
  });

  describe("extension in base classes should not cause to exception", () => {
    beforeEach(() => {
      Object.prototype.objectExtraField = () => {};
      Array.prototype.arrayExtraField = () => {};
    });
    afterEach(() => {
      delete Object.prototype.objectExtraField;
      delete Array.prototype.arrayExtraField;
    });

    it("fork()", () => {
      expect(() => {
        fork({
          node: {
            Test: {
              structure: {
                foo: "Rule",
                bar: [["Rule"]],
              },
            },
          },
        });
      }).not.toThrow();
    });
  });

  it("generic option should work in fork()", () => {
    const forkWithoutGeneric = fork({
      // By default, generic is true, but we disable it
      generic: false,
    });

    const forkWithGeneric = fork({
      // By default, generic is true
    });

    // Generic should be set
    expect(forkWithoutGeneric.lexer.generic).toBe(false);
    expect(forkWithGeneric.lexer.generic).toBe(true);

    // Lexer match should depend on generic (<length> is generic type)
    const node = parse("1px", { context: "value" });

    expect(() => forkWithoutGeneric.lexer.match("<length>", node)).toThrow();
    expect(() => forkWithGeneric.lexer.match("<length>", node)).not.toThrow();
  });

  describe("fork()", () => {
    it("extend nodes", () => {
      const ast = parse("20px", { context: "value" });
      const forkedSyntax = fork({
        node: {
          Dimension: {
            generate(node) {
              this.token(tokenTypes.Dimension, "?" + node.unit);
            },
          },
        },
      });

      // Generic should be set
      expect(forkedSyntax.generate(ast)).toBe("?px");
      expect(generate(ast)).toBe("20px");
    });
  });
});

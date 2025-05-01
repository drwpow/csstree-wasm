import { describe, expect, it } from "vitest";
import { parse, find, findLast, findAll } from "../pkg/csstree_wasm.js";
import { lazyValues } from "./helpers/index.js";

const values = lazyValues({
  ast: () =>
    parse(`
        .foo { color: red; background: green; }
        .bar, .qux.foo { font-weight: bold; color: blue; }
    `),
  firstFoo: () =>
    values.ast.children.first.prelude.children.first.children.first, // Rule // SelectorList // Selector // ClassSelector
  lastFoo: () => values.ast.children.last.prelude.children.last.children.last, // Rule // SelectorList // Selector // ClassSelector
});

describe("Search", () => {
  describe("find", () => {
    it("base", () => {
      const actual = find(
        values.ast,
        (node) => node.type === "ClassSelector" && node.name === "foo",
      );

      expect(actual).toEqual(values.firstFoo);
    });

    it("using refs", () => {
      const actual = find(
        values.ast,
        (node, item, list) =>
          node.type === "ClassSelector" &&
          node.name === "foo" &&
          list.head !== item,
      );

      expect(actual).toEqual(values.lastFoo);
    });

    it("using context", () => {
      const actual = find(values.ast, function (node) {
        return (
          node.type === "ClassSelector" &&
          node.name === "foo" &&
          this.selector.children.head !== this.selector.children.tail
        );
      });

      expect(actual).toEqual(values.firstFoo);
    });
  });

  describe("findLast", () => {
    it("findLast", () => {
      const actual = findLast(
        values.ast,
        (node) => node.type === "ClassSelector" && node.name === "foo",
      );

      expect(actual).toEqual(values.lastFoo);
    });

    it("using refs", () => {
      const actual = findLast(
        values.ast,
        (node, item, list) =>
          node.type === "ClassSelector" &&
          node.name === "foo" &&
          list.head === item,
      );

      expect(actual).toEqual(values.firstFoo);
    });

    it("using context", () => {
      const actual = findLast(values.ast, function (node) {
        return (
          node.type === "ClassSelector" &&
          node.name === "foo" &&
          this.selector.children.head === this.selector.children.tail
        );
      });

      expect(actual).toEqual(values.firstFoo);
    });
  });

  it("findAll", () => {
    const actual = findAll(
      values.ast,
      (node) => node.type === "ClassSelector" && node.name === "foo",
    );

    expect(actual).toHaveLength(2);
    expect(actual[0]).toEqual(values.firstFoo);
    expect(actual[1]).toEqual(values.lastFoo);
  });
});

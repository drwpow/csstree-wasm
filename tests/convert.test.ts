import { describe, expect, it } from "vitest";
import {
  parse,
  generate,
  fromPlainObject,
  toPlainObject,
} from "../pkg/csstree_wasm.js";

const css = ".test{a:123}";

describe("convert", () => {
  it("fromPlainObject", () => {
    const ast = parse(css);
    const plainObject = JSON.parse(JSON.stringify(ast));
    const actual = generate(fromPlainObject(plainObject));

    expect(actual).toEqual(css);
  });

  it("toPlainObject", () => {
    const ast = parse(css);
    const expected = JSON.parse(JSON.stringify(ast));
    const actual = toPlainObject(ast);

    expect(actual).toEqual(expected);
  });
});

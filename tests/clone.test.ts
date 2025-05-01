import { describe, expect, it } from "vitest";
import { clone, parse, toPlainObject, walk } from "../pkg/csstree_wasm.js";

function createCloneTest(name, getAst) {
  it(name, () => {
    const ast = getAst();
    const astCopy = clone(ast);
    const astNodes = [];
    let clonedNodeCount = 0;
    let nonClonedNodeCount = 0;

    walk(ast, (node) => astNodes.push(node));
    walk(astCopy, (node) => {
      clonedNodeCount++;
      nonClonedNodeCount += astNodes.includes(node);
    });

    expect(clonedNodeCount).toBe(astNodes.length);
    expect(nonClonedNodeCount).toBe(0);
  });
}

describe("clone()", function () {
  createCloneTest("a regular AST", () =>
    parse(".test{color:red;}@media foo{div{color:green}}"),
  );

  createCloneTest("an AST as JSON", () =>
    toPlainObject(parse(".test{color:red;}@media foo{div{color:green}}")),
  );
});

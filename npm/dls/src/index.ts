import fs from "fs";
import path from "path";
import { ts, tsx } from "@ast-grep/napi";

function getLang(file: string) {
  switch (path.extname(file)) {
    case ".ts":
      return ts;
    case ".tsx":
      return tsx;
    default:
      throw new Error("unsupported file type");
  }
}

export async function findDeps(entry: string) {
  const code = await fs.promises.readFile(entry, "utf-8");
  const deps = getLang(entry)
    .parse(code)
    .root()
    .findAll("const $A = $B")
    .map((node) => {
      const { start, end } = node.range();
      return code.slice(start.index, end.index);
    });

  return deps;
}

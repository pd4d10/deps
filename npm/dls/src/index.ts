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

export type TreeNode = {
  path: string;
  children: TreeNode[];
  circular: boolean;
};

function resolveFile(specifier: string) {
  for (const suffix of [
    ".js",
    ".jsx",
    ".ts",
    ".tsx",
    "/index.js",
    "/index.jsx",
    "/index.ts",
    "/index.tsx",
  ]) {
    const fullPath = specifier + suffix;
    if (fs.existsSync(fullPath)) return fullPath;
  }

  throw new Error("file not exists: " + specifier);
}

export async function collectDeps(
  entry: string,
  parent: TreeNode,
  files: Set<string>
) {
  console.log("[scanning]", entry);

  const circular = files.has(entry);
  const current: TreeNode = {
    path: entry,
    children: [],
    circular,
  };

  parent.children.push(current);
  files.add(entry);

  if (!circular) {
    const code = await fs.promises.readFile(entry, "utf-8");
    const deps = getLang(entry)
      .parse(code)
      .root()
      .findAll('import $_ from "$PATH"')
      .flatMap((node) => {
        const match = node.getMatch("PATH");
        return match ? [match.text()] : [];
      });

    for (const dep of deps) {
      if (!dep.startsWith(".")) continue;

      const absPath = resolveFile(path.resolve(path.dirname(entry), dep));
      await collectDeps(absPath, current, files);
    }
  }
}

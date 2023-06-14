import cac from "cac";
import { TreeNode, collectDeps } from ".";

const cli = cac();

cli.command("dep [file]").action(async (file: string) => {
  const root: TreeNode = {
    path: file,
    children: [],
    circular: false,
  };
  const files = new Set<string>();
  await collectDeps(file, root, files);
  console.log(JSON.stringify(root, null, 2));
});

cli.help();
cli.version(require("../package.json").version);
cli.parse();

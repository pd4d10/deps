import cac from "cac";
import { findDeps } from ".";

const cli = cac();

cli.command("dep [file]").action(async (file: string) => {
  const deps = await findDeps(file);
  console.log(deps);
});

cli.help();
cli.version(require("../package.json").version);
cli.parse();

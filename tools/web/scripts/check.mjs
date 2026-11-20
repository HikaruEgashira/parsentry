// Syntax-check the UI scripts and enforce the rendering/token rules:
// no HTML sinks (only textContent) and no persistent token storage.
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";
import vm from "node:vm";

const dir = new URL("../public/", import.meta.url).pathname;
const banned = [/\.innerHTML\b/, /\.outerHTML\b/, /insertAdjacentHTML/, /document\.write/, /\beval\(/, /localStorage/];
let failed = false;

for (const name of readdirSync(dir).filter((n) => n.endsWith(".js"))) {
  const src = readFileSync(join(dir, name), "utf8");
  try {
    new vm.Script(src, { filename: name });
  } catch (e) {
    console.error(`${name}: ${e.message}`);
    failed = true;
  }
  for (const re of banned) {
    if (re.test(src)) {
      console.error(`${name}: forbidden pattern ${re}`);
      failed = true;
    }
  }
}

const html = readFileSync(join(dir, "index.html"), "utf8");
if (/<script(?![^>]*\bsrc=)[^>]*>/i.test(html) || /\son\w+=/i.test(html)) {
  console.error("index.html: inline scripts or handlers are not allowed");
  failed = true;
}

if (failed) process.exit(1);
console.log("web UI check passed");

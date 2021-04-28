import { Plug } from "https://deno.land/x/plug/mod.ts";
import { dirname } from "https://deno.land/std@0.91.0/path/mod.ts";

const __dirname = dirname(import.meta.url);


const options= {
  name: "test_plugin",
  url: `${__dirname}/../target/debug`
};

const rid = await Plug.prepare(options);
const enc = new TextEncoder();
const { testSync } = Plug.core.ops();
const response = Plug.core.dispatch(
    testSync,
    enc.encode("hello!")
);
const dec = new TextDecoder();
console.log(dec.decode(response))
Deno.close(rid);
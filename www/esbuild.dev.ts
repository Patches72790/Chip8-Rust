import esbuild from "esbuild";
import wasmLoader from "esbuild-plugin-wasm";

await esbuild.build({
  entryPoints: ["./bootstrap.js"],
  bundle: true,
  outfile: "./dist/bootstrap.js",
  plugins: [wasmLoader()],
});

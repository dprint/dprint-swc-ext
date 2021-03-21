// see https://doc.rust-lang.org/cargo/commands/cargo-metadata.html
const cmd = Deno.run({
    cmd: ["cargo", "metadata", "--format-version", "1"],
    cwd: "./rs-lib",
    stdout: "piped",
});

try {
    const output = JSON.parse(new TextDecoder().decode(await cmd.output()));
    const swc_ecma_ast = output.packages.find((pkg: any) => pkg.name === "swc_ecma_ast");
    console.log(swc_ecma_ast.version);
} finally {
    cmd.close();
}

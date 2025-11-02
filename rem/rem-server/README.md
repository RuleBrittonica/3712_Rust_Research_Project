This server handles 5 operations:

init { manifestPath }

create { path, text? }

change { path, text? }

delete { path }

extract { file, newFnName, start, end }

Request examples:
{"op":"init","manifestPath":"/abs/path/Cargo.toml"}
{"op":"create","path":"/abs/src/lib.rs","text":"fn x() {}"}
{"op":"change","path":"/abs/src/lib.rs","text":"fn x(){println!(\"hi\");}"}
{"op":"delete","path":"/abs/src/old.rs"}
{"op":"extract","file":"/abs/src/main.rs","newFnName":"extracted_function","start":39,"end":60}


Responses:
{"ok":true,"data":{"status":"initialized"}}
{"ok":true,"data":{"status":"applied","path":"/abs/src/lib.rs"}}
{"ok":false,"error":"explain what failed"}

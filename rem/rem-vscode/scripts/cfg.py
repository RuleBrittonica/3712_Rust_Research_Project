#!/usr/bin/env python3
import sys

import tomllib

def load_cfg(path="config.toml"):
    with open(path, "rb") as f:
        return tomllib.load(f)

def merged_packages(cfg, os_key):
    pk = cfg.get("opam", {}).get("packages", {})
    common = pk.get("common", []) or []
    os_specific = pk.get(os_key, []) or []
    seen = set()
    out = []
    for p in list(common) + list(os_specific):
        if p not in seen:
            seen.add(p)
            out.append(p)
    return out

def export_shell(cfg, os_key):
    ocaml = cfg.get("ocaml", {})
    switch = ocaml.get("version", "")
    variant = ocaml.get("variant", "")
    pkgs = merged_packages(cfg, os_key)

    if not switch:
        print('echo "cfg.py: missing [ocaml].version in config.toml" >&2')
        sys.exit(2)

    print(f'SWITCH="{switch}"')
    if variant:
        print(f'OCAML_VARIANT="{variant}"')
    # space-separated list for POSIX shells
    print('OPAM_PACKAGES="' + " ".join(pkgs) + '"')

    # URLs -> uppercased variable names
    for k, v in (cfg.get("urls") or {}).items():
        print(f'{k.upper()}="{v}"')

    a = (cfg.get("aeneas") or {})
    if "repo" in a:
        print(f'AENEAS_REPO="{a["repo"]}"')
    if "ref" in a:
        print(f'AENEAS_REF="{a["ref"]}"')


def export_powershell(cfg, os_key):
    ocaml = cfg.get("ocaml", {})
    switch = ocaml.get("version", "")
    variant = ocaml.get("variant", "")
    pkgs = merged_packages(cfg, os_key)

    if not switch:
        sys.stderr.write("cfg.py: missing [ocaml].version in config.toml\n")
        sys.exit(2)

    print(f'$switch = "{switch}"')
    if variant:
        print(f'$ocamlVariant = "{variant}"')
    else:
        print('$ocamlVariant = ""')
    print("$opamPackages = @(" + ", ".join(f'"{p}"' for p in pkgs) + ")")

    for k, v in (cfg.get("urls") or {}).items():
        print(f'${k} = "{v}"')  # keep original casing for PS vars

    a = (cfg.get("aeneas") or {})
    if "repo" in a:
        print(f'$aeneasRepo = "{a["repo"]}"')
    if "ref" in a:
        print(f'$aeneasRef = "{a["ref"]}"')


def usage():
    print("Usage: cfg.py export-shell <linux|macos|windows> | export-powershell <windows>")
    sys.exit(2)

def main():
    if len(sys.argv) != 3:
        usage()
    cmd, os_key = sys.argv[1], sys.argv[2].lower()
    if os_key not in ("linux", "macos", "windows"):
        usage()
    cfg = load_cfg()
    if cmd == "export-shell":
        export_shell(cfg, os_key)
    elif cmd == "export-powershell":
        export_powershell(cfg, os_key)
    else:
        usage()

if __name__ == "__main__":
    main()

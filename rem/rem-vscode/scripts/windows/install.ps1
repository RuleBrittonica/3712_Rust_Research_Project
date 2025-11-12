<#
.SYNOPSIS
  Install the full REM/Aeneas/CHARON toolchain natively on Windows 10+.

.DESCRIPTION
  1) Installs Git and OPAM (with OCaml & Coq) via WinGet.
  2) Inits OPAM, creates switch, installs Dune & libraries (from config.toml).
  3) Installs Rust nightly + components via rustup.
  4) Uses OPAM’s MSYS2 bash to build Aeneas & CHARON.
  5) Copies binaries → %USERPROFILE%\bin.
  6) Installs rem-server via cargo.
#>

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

function Write-Ok($msg){ Write-Host "$msg" -ForegroundColor Green }
function Write-Warn($msg){ Write-Host "$msg" -ForegroundColor Yellow }

# 0) Load configuration from config.toml
#    Exports: $switch, $ocamlVariant, $opamPackages, $aeneasRepo, $aeneasRef
python tools/cfg.py export-powershell windows | Invoke-Expression

# 1) Ensure WinGet is available
if (-not (Get-Command winget -ErrorAction SilentlyContinue)) {
  Write-Warn "WinGet not found. Please install the 'App Installer' from the Microsoft Store."
  exit 1
}

# 2) Install Git & OPAM via WinGet
Write-Host "==> Installing Git and OPAM (OCaml) via WinGet…"
winget install --exact --id Git.Git --silent --accept-package-agreements
winget install --exact --id OCaml.opam --silent --accept-package-agreements
Write-Ok "Git and OPAM installed"

# 3) Bootstrap OPAM
Write-Host "==> Initializing OPAM…"
& opam init --disable-sandboxing -y | Out-Null
& opam env --shell=powershell | Invoke-Expression

# 3.a) Create or switch to configured OCaml version
Write-Host "==> Creating / switching to OPAM switch $switch…"
if (-not (& opam switch list --short | Select-String "^$switch$")) {
  if ($ocamlVariant) {
    & opam switch create $switch $ocamlVariant --yes
  } else {
    & opam switch create $switch --yes
  }
} else {
  & opam switch $switch
}
& opam env --shell=powershell | Invoke-Expression

# 3.b) Install OCaml packages from config.toml
Write-Host "==> Installing OCaml platform libraries via OPAM…"
& opam update --yes | Out-Null
& opam upgrade --yes | Out-Null
& opam install -y @opamPackages

# Coq handling — skip if on OCaml 5.x and install standalone Coq via WinGet
$reqCoq = "8.18.0"
$ocamlVer = (& ocamlc -version)
if ($ocamlVer.Split('.')[0] -ge 5) {
  Write-Warn "OCaml $ocamlVer detected — skipping Coq $reqCoq (may not support OCaml 5.x)."
  Write-Host "==> Installing standalone Coq (via WinGet)…"
  winget install --exact --id coq.coq --silent --accept-package-agreements
  Write-Ok "Coq installed via WinGet"
} else {
  $coqVer = (& coqc --version 2>$null) -replace '[^\d\.]',''
  if ($coqVer -ne $reqCoq) {
    Write-Host "==> Installing Coq $reqCoq via OPAM…"
    & opam pin add coq $reqCoq --yes --no-action
    & opam install -y coq
  }
}
Write-Ok "OPAM, OCaml, Coq and libraries ready"

# 4) Install Rust + nightly toolchain
$rustupExe = "$env:USERPROFILE\.cargo\bin\rustup.exe"
if (-not (Test-Path $rustupExe)) {
  Write-Host "==> Installing rustup…"
  $installer = "$env:TEMP\rustup-init.exe"
  Invoke-WebRequest -Uri "https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe" `
                    -OutFile $installer
  Start-Process -FilePath $installer -ArgumentList "-y" -NoNewWindow -Wait
}
# Ensure Cargo & rustup are on PATH
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

$toolchain = "nightly"
Write-Host "==> Installing Rust toolchain $toolchain…"
rustup toolchain install $toolchain --profile minimal
"rust-src","rust-std","rustc-dev","llvm-tools-preview","rust-analyzer-preview","rustfmt" `
  | ForEach-Object { rustup component add $_ --toolchain $toolchain }
Write-Ok "Rust $toolchain + components ready"

# 5) Clone & build Aeneas/CHARON under MSYS2
$cacheDir = "$env:LOCALAPPDATA\aeneas"
$repo = $aeneasRepo
$ref  = $aeneasRef

Write-Host "==> Cloning/updating Aeneas from $repo…"
if (-not (Test-Path "$cacheDir\.git")) {
  git clone $repo $cacheDir
} else {
  git -C $cacheDir remote set-url origin $repo
  git -C $cacheDir fetch --all --tags
}

if ($ref) {
  Write-Host "==> Checking out Aeneas @ $ref…"
  git -C $cacheDir fetch --all --tags
  git -C $cacheDir checkout --detach $ref
} else {
  Write-Host "==> No AENEAS_REF specified; using current default branch."
}

# Locate MSYS2 bash from OPAM
$opamRoot = (& opam var root).Trim()
$bashPath = Join-Path $opamRoot "msys2\usr\bin\bash.exe"
if (-not (Test-Path $bashPath)) {
  Write-Warn "Could not find MSYS2 bash at $bashPath"
} else {
  $drive = $cacheDir.Substring(0,1).ToLower()
  $msysPath = "/$drive" + ($cacheDir.Substring(2) -replace '\\','/')
  Write-Host "==> Building CHARON & Aeneas in MSYS2…"
  & $bashPath -lc "
    cd `"$msysPath`"
    make setup-charon
    make
    make test
  "
  Write-Ok "Built Aeneas & CHARON"
}

# 6) Copy binaries → %USERPROFILE%\bin
$binDir = "$env:USERPROFILE\bin"
if (-not (Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir | Out-Null }

@("bin/aeneas.exe","charon/bin/charon.exe","charon/bin/charon-driver.exe") `
  | ForEach-Object {
      $src = Join-Path $cacheDir ($_ -replace '/','\')
      if (Test-Path $src) {
        Copy-Item $src -Destination $binDir -Force
        Write-Host "Installed $([IO.Path]::GetFileName($src)) → $binDir"
      } else {
        Write-Warn "$src not found"
      }
    }

# 7) Install rem-server via Cargo (to match Linux/macOS)
if (-not (Get-Command rem-server -ErrorAction SilentlyContinue)) {
  Write-Host "==> Installing rem-server via cargo…"
  cargo install rem-server --locked
  Write-Ok "rem-server installed"
} else {
  Write-Ok "rem-server already present"
}

Write-Host "`nAll done!"
Write-Host "Add `$env:USERPROFILE\bin` (and `$env:USERPROFILE\.cargo\bin`) to your User PATH if you haven't already."
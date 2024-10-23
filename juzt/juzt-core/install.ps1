#Requires -RunAsAdministrator

# Check if Rust is installed
if (-not (Get-Command rustc -ErrorAction SilentlyContinue)) {
    Write-Output "Rust is not installed. Installing Rust..."
    Invoke-WebRequest -Uri https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
    .\rustup-init.exe -y
    Remove-Item rustup-init.exe
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
}

# Check if Nushell is installed
if (-not (Get-Command nu -ErrorAction SilentlyContinue)) {
    Write-Output "Installing Nushell..."
    cargo install nu
}

Write-Output "Nushell installation complete."
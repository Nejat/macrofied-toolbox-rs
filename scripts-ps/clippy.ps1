function Confirm-Success {
    param([string] $step)

    if ($lastExitCode -ne 0) {
        Write-Host
        Write-Host "$step failed with exit code $lastExitCode" -ForegroundColor Red
        Write-Host
        exit $lastExitCode
    }
}

[string] $script = $PSCommandPath.Split([IO.Path]::DirectorySeparatorChar)[-1];

if (Test-Path -Path $script -PathType Leaf) {
    Write-Host
    Write-Host "Scripts need to be run from the project root, i.e. '.\scripts-ps\$script'" -ForegroundColor Yellow
    Write-Host

    exit -1
}

clear
Write-Host "running clippy option unoptimized" -ForegroundColor Yellow
cargo clippy --features="option"
Confirm-Success "clippy option unoptimized"

Write-Host "running clippy option optimized" -ForegroundColor Yellow
cargo clippy --release --features="option"
Confirm-Success "clippy option optimized"

Write-Host "running clippy option-debug unoptimized" -ForegroundColor Yellow
cargo clippy --features="option-debug"
Confirm-Success "clippy option-debug unoptimized"

Write-Host "running clippy option-debug optimized" -ForegroundColor Yellow
cargo clippy --release --features="option-debug"
Confirm-Success "clippy option-debug optimized"

Write-Host "running clippy result unoptimized" -ForegroundColor Yellow
cargo clippy --features="result"
Confirm-Success "clippy result unoptimized"

Write-Host "running clippy result optimized" -ForegroundColor Yellow
cargo clippy --release --features="result"
Confirm-Success "clippy result optimized"

Write-Host "running clippy result-debug unoptimized" -ForegroundColor Yellow
cargo clippy --features="result-debug"
Confirm-Success "clippy result-debug unoptimized"

Write-Host "running clippy result-debug optimized" -ForegroundColor Yellow
cargo clippy --release --features="result-debug"
Confirm-Success "clippy result-debug optimized"

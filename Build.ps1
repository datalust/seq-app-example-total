param (
    [string] $version = "1.0.0-local-$([System.DateTime]::UtcNow.ToString("yyyyMMddhhmmss"))"
)

if (Test-Path .\publish) {
    rm .\publish -Recurse
}

mkdir .\publish
cargo build --release
& .\tool\nuget.exe pack .\Seq.App.Example.Total.nuspec -version $version -outputdirectory .\publish

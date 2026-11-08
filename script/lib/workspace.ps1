
function ParseSotidWorkspace {
    $metadata = cargo metadata --no-deps --offline | ConvertFrom-Json
    $env:SOTID_WORKSPACE = $metadata.workspace_root
    $env:RELEASE_VERSION = $metadata.packages | Where-Object { $_.name -eq "sotid" } | Select-Object -ExpandProperty version
}

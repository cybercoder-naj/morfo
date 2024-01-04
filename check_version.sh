mainVersion=$(git show main:Cargo.toml | grep -oP 'version = "\K[^"]+')
currentVersion=$(grep -oP 'version = "\K[^"]+' Cargo.toml)

test "$(echo "$currentVersion $mainVersion" | tr " " "\n" | sort -V | head -n 1)" != "$currentVersion"

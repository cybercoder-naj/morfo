mainVersion=$(git show main:Cargo.toml | grep -oP '^version = "\K[^"]+')
echo "Main version: $mainVersion"
currentVersion=$(grep -oP '^version = "\K[^"]+' Cargo.toml)
echo "Current branch's version: $currentVersion"

if test "$(echo "$currentVersion $mainVersion" | tr " " "\n" | sort -V | head -n 1)" != "$currentVersion"; then
  echo "Ok!"
else
  echo "Version is not greater than that of main branch"
  exit 1
fi

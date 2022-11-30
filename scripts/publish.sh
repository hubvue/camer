set -e

for package in npm/*; do
  echo "$package/package.json"
  if [ -f "$package/package.json" ]; then
    pnpm publish $package --tag latest --access publish
  fi;
done

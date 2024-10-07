wasm-pack build --scope class-undefined --target web
cp -f package.json pkg/package.json
cd pkg
npm config set registry https://registry.npmjs.org
npm publish --access=public
npm config set registry https://registry.npmmirror.com
cd ..
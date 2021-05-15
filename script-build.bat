cargo build

cd libpxlr
wasm-pack build --target web
cd ..

cd webapp
npm run build
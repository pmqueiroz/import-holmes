pnpm build
package=$(pnpm pack)
npm remove -g "import-holmes"
npm install -g "file:./$package"
rm -rf $package

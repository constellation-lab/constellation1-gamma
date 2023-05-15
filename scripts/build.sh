
build_wasm()
{
    cargo build --release
}

build_docs()
{
    cargo doc
}

build_all()
{
    build_wasm
    build_docs
}

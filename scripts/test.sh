test.sh

test_unit()
{
    cargo test
}

test_integration()
{
    cargo test --features integration
}

test_all()
{
    test_unit
    test_integration
}

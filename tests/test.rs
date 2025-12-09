use std::fs;

#[test]
fn test_sync_code() {
    sync_code::Builder::new()
        .add("tests/data1/target.txt", "tests/data1/source.txt")
        .add("tests/data2/target.txt", "tests/data2/source.txt")
        // Chain dependence
        .add("tests/data3/source2.txt", "tests/data3/source.txt")
        .add("tests/data3/target.txt", "tests/data3/source2.txt")
        .add("tests/data4_lf/source2.txt", "tests/data4_lf/source.txt")
        .add("tests/data4_lf/target.txt", "tests/data4_lf/source2.txt")
        .sync();

    assert_eq!(
        fs::read("tests/data1/target.txt").unwrap(),
        fs::read("tests/data1/source.txt").unwrap()
    );
    assert_eq!(
        fs::read("tests/data2/target.txt").unwrap(),
        fs::read("tests/data2/expected.txt").unwrap()
    );
    assert_eq!(
        fs::read("tests/data3/target.txt").unwrap(),
        fs::read("tests/data3/expected.txt").unwrap()
    );
    assert_eq!(
        fs::read("tests/data4_lf/target.txt").unwrap(),
        fs::read("tests/data4_lf/expected.txt").unwrap()
    );
}

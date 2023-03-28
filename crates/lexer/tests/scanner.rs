use pai_fs_loader::FileLoader;
use pai_lexer::Lexer;

#[test]
fn debug_scanner() {
    let loader = FileLoader::load("./tests/fixtures/demo.ts").unwrap();

    let src = loader.source_code();

    let Lexer { mut scanner, .. } = Lexer::new(src);

    println!("{:?}", scanner.scan_hashbang());

    // println!("{:?}", scanner.next());

    while let Some(unit) = scanner.next() {
        println!("{:?}", unit)
    }
}

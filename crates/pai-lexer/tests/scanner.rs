use insta::assert_snapshot;
use pai_fs::SourceFile;
use pai_lexer::Lexer;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[test]
fn demo() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let sf = SourceFile::read("./tests/fixtures/demo.ts").unwrap();

    let lexer = Lexer::new(sf.source());

    // println!("{:?}", lexer.scanner.scan_hashbang());
    // println!("{:X}", unsafe { *lexer.scanner.end });

    let output: String = lexer.map(|unit| format!("{unit:?}\n")).collect();

    assert_snapshot!(output);
}

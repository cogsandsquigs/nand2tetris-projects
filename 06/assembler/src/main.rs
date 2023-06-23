use assembler::Assembler;

fn main() {
    println!("Hello, world!");

    let mut assembler = Assembler::new(std::io::BufReader::new(
        std::fs::File::open("test.asm").unwrap(),
    ));

    let binary = assembler.assemble().unwrap();

    println!("{:?}", binary);
}

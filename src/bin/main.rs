use ex_3_1::binary::Binary;

fn main() {
    let binary = Binary::new(9);
	for bits in binary {
		println!("{:?}", bits);
	}
}




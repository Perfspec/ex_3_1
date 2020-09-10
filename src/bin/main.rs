use ex_3_1::PowerSet;

fn main() {
    let power_set = PowerSet::new(4);
	println!("{}", power_set.count_topologies());
}




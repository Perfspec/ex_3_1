#[macro_use]
extern crate matches;

pub mod binary;

//use std::convert::TryInto;
//use std::fmt;

//#[derive(Debug)]
//pub struct PowerSet {
//	power_set: Vec<Vec<usize>>,
//}

//impl PowerSet {
	
//	pub fn new(cardinality: usize) -> PowerSet
//	{
//		assert!(cardinality < 16);
//		
//		let power_set_cardinality: usize = 2_usize.pow(cardinality.try_into().unwrap());
//		let mut power_set = Vec::with_capacity(power_set_cardinality);
//		let binary_counter = binary::Binary::new(cardinality);
//		for binary in binary_counter {
//			let binary_bools = binary.get();
//			let mut subset = Vec::new();
//			for i in 0..cardinality {
//				if let Some(binary_bool) = binary_bools.get(i) {
//					if *binary_bool {
//						subset.push(i);
//					}
//				}
//			}
//			println!("{:#?}", &subset);
//			power_set.push(subset);
//		}
//		PowerSet {
//			power_set
//		}
//	}
	
//	fn count_topologies(&self) -> usize {
//		0
//	}
//}

//impl fmt::Debug for PowerSet {
//    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
//        fmt.debug_set().entries(self.power_set.iter()).finish()
//    }
//}

//#[cfg(test)]
//mod tests {
//	use super::*;
	
//}
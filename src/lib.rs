#[macro_use]
extern crate matches;

mod binary;

use std::convert::TryInto;
use std::fmt;

pub struct PowerSet {
	cardinality: usize,
	power_set: Vec<Vec<usize>>,
	power_set_cardinality: usize,
}

impl PowerSet {
	
	pub fn new(cardinality: usize) -> PowerSet
	{
		assert!(cardinality < 16);
		
		let power_set_cardinality: usize = 2_usize.pow(cardinality.try_into().unwrap());
		let mut power_set = Vec::with_capacity(power_set_cardinality);
		power_set.push(Vec::new());
		let binary_counter = binary::Binary::new(cardinality);
		for binary in binary_counter {
			let binary_bools = binary.get();
			let mut subset = Vec::new();
			for i in 0..cardinality {
				if let Some(binary_bool) = binary_bools.get(i) {
					if *binary_bool {
						subset.push(i);
					}
				}
			}
			power_set.push(subset);
		}
		PowerSet {
			cardinality,
			power_set,
			power_set_cardinality,
		}
	}
	
	pub fn count_topologies(&self) -> usize {
		let binary_counter = binary::Binary::new(self.power_set_cardinality);
		let mut count_of_topologies: usize = 0;
		
		// check all possible subsets of the power_set to see if they are topologies or not
		for binary in binary_counter {
			println!(".");
			if self.is_topology(&binary) {
				count_of_topologies += 1;
			}
		}
		count_of_topologies
	}
	
	fn is_topology(&self, binary: &binary::Binary) -> bool {
		// generate subset of power_set
		let mut power_set_subset = Vec::new();
		let binary_bools = binary.get();
		for i in 0..self.power_set_cardinality {
			if let Some(binary_bool) = binary_bools.get(i) {
				if *binary_bool {
					match self.power_set.get(i) {
						Some(subset) => {
							power_set_subset.push(subset.clone());
						},
						None => {
							panic!("Error in fn is_topology from struct PowerSet: cannot retrieve element from power_set");
						},
					}
				}
			}
		}
		
		// validate subset of power_set
		let empty_set: Vec<usize> = Vec::new();
		let mut whole_set = Vec::new();
		for i in 0..self.cardinality {
			whole_set.push(i);
		}
		let contains_empty_set = power_set_subset.contains(&Vec::new());
		let contains_whole_set = power_set_subset.contains(&whole_set);
		let mut unions_are_conserved = true;
		let mut intersections_are_conserved = true;
		if contains_empty_set & contains_whole_set {
			for element1 in &power_set_subset {
				for element2 in &power_set_subset {
					let mut union = [element1.clone(),element2.clone()].concat();
					union.sort();
					union.dedup();
					unions_are_conserved = unions_are_conserved & (power_set_subset.contains(&union));
					let mut intersection = Vec::new();
					for &i in element1 {
						if element2.contains(&i) {
							intersection.push(i);
						}
					}
					intersections_are_conserved = intersections_are_conserved & (power_set_subset.contains(&intersection));
				}
			}
		} else {
			unions_are_conserved = false;
			intersections_are_conserved = false;
		}
		contains_empty_set & contains_whole_set & unions_are_conserved & intersections_are_conserved
	}
}

impl fmt::Debug for PowerSet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_set().entries(self.power_set.iter()).finish()
	}
}

//#[cfg(test)]
//mod tests {
//	use super::*;
	
//}
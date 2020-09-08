use std::fmt;

pub struct Binary {
	binary: Vec<bool>
}

impl Binary {
	pub fn new(len: usize) -> Binary {
		assert!(len > 0);
		let mut all_false = Vec::with_capacity(len);
		for _i in 0..len {
			all_false.push(false);
		}
		Binary {
			binary: all_false
		}
	}
	
	pub fn get(&self) -> &Vec<bool> {
		&self.binary
	}
	
}

impl Clone for Binary {
	fn clone(&self) -> Self {
		Binary {
			binary: self.binary.clone()
		}
	}
}

impl Iterator for Binary {
	type Item = Binary;
	
	/// increment a fixed-length binary number here. return the next binary if it is not all ones, else return none.
	fn next(&mut self) -> Option<Self::Item> {
		
		let mut index_of_first_false: usize = 0;
		
		'find_index: for &boolean in &self.binary {
			if boolean {
				index_of_first_false += 1;
			} else {
				break 'find_index;
			}
		}
		
		if index_of_first_false == 0 {
			match self.binary.get_mut(0) {
				Some(first) => {
					*first = true;
				},
				None => {
					panic!("Error in trait Iterator for struct Binary: cannot find the next item when the Binary is empty");
				}
			}
			Some(self.clone())
			
		} else if index_of_first_false < self.binary.len() {
			let mut all_false = [false].repeat(index_of_first_false);
			self.binary[..(index_of_first_false)].swap_with_slice(&mut all_false);
			
			match self.binary.get_mut(index_of_first_false) {
				Some(first) => {
					*first = true;
				},
				None => {
					panic!("Error in trait Iterator for struct Binary: cannot find the next item when ");
				}
			}
			Some(self.clone())
			
		} else {
			None
		}
	}
}

impl fmt::Debug for Binary {
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		let mut output = String::new();
		for &boolean in &self.binary {
			if boolean {
				output.push_str("1");
			} else {
				output.push_str("0");
			}
		}
		fmt.write_str(&output)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn when_new_binary_of_size_three_then_return_all_false() {
		let new_binary = Binary::new(3);
		let mut result = true;
		for should_be_false in new_binary.get() {
			result = result & (!should_be_false);
		}
        assert!(result);
    }
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_once_then_first_element_is_true_and_other_elements_are_false() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_twice_then_first_element_is_false_second_is_true_and_third_is_false() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_three_times_then_first_element_is_true_second_is_true_and_third_is_false() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_four_times_then_first_element_is_false_second_is_false_and_third_is_true() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_five_times_then_first_element_is_true_second_is_false_and_third_is_true() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_six_times_then_first_element_is_false_second_is_true_and_third_is_true() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		
		let mut iter_bools = new_binary.get().iter();
		
		// first element should be false
		match iter_bools.next() {
			Some(should_be_false) => {
				assert!(!should_be_false);
			},
			None => {
				panic!("Error in tests for struct Binary: first element in vector of booleans is missing");
			}
		}
		
		// second element should be false
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: second element in vector of booleans is missing");
			}
		}
		
		// third element should be true
		match iter_bools.next() {
			Some(should_be_true) => {
				assert!(should_be_true);
			},
			None => {
				panic!("Error in tests for struct Binary: third element in vector of booleans is missing");
			}
		}
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_seven_times_then_all_elements_are_true() {
		let mut new_binary = Binary::new(3);
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		new_binary.next();
		
		let mut result = true;
		for should_be_true in new_binary.get() {
			result = result & should_be_true;
		}
        assert!(result);
	}
	
	#[test]
	fn when_new_binary_of_size_one_then_element_is_false() {
		let new_binary = Binary::new(1);
		let this_bool = new_binary.get().get(0);
		assert_matches!(this_bool, Some(false));
	}
	
	#[test]
	fn when_new_binary_of_size_one_and_goto_next_once_then_element_is_true() {
		let mut new_binary = Binary::new(1);
		assert_matches!(new_binary.next(), Some(_));
		let this_bool = new_binary.get().get(0);
		assert_matches!(this_bool, Some(true));
	}
	
	#[test]
	fn when_new_binary_of_size_three_and_goto_next_eight_times_then_return_none() {
		let mut new_binary = Binary::new(3);
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), Some(_));
		assert_matches!(new_binary.next(), None);
	}
}
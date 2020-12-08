///! A set of useful methods and abstractions.

use crate::ERC20Error;
use hex::FromHexError;
use web3::types::{
	Bytes,
	H160,
	H256,
	U256,
};

/// Converts `Bytes` and `Vec<u8>` to H160, H256, and U256.
pub struct BytesToFixedNumber {
	data: Vec<u8>,
	index: usize,
}

impl From<Vec<u8>> for BytesToFixedNumber {
	#[inline]
	fn from(data: Vec<u8>) -> Self {
		Self {
			data,
			index: 0,
		}
	}
}

impl From<Bytes> for BytesToFixedNumber {
	#[inline]
	fn from(data: Bytes) -> Self {
		data.0.into()
	}
}

impl BytesToFixedNumber {
	/// Returns the next vector for the specified size.
	///
	/// # Arguments
	///
	/// * `size` - The size requested for the next vector.
	///
	pub fn next_vec(&mut self, size: usize) -> Result<Vec<u8>, ERC20Error> {
		if self.index + size > self.data.len() {
			return Err(ERC20Error::UnexpectedEndOfData);
		}
		let mut resp = Vec::new();
		for i in 0..size {
			resp.push(self.data[self.index + i]);
		}
		self.index += size;
		Ok(resp)
	}

	/// Skips a specified number of bytes.
	///
	/// # Arguments
	///
	/// * `size` - The number of bytes to skip.
	///
	pub fn skip(&mut self, size: usize) -> Result<(), ERC20Error> {
		if self.index + size > self.data.len() {
			return Err(ERC20Error::UnexpectedEndOfData);
		}
		self.index += size;
		Ok(())
	}

	/// Returns the next H160.
	pub fn next_h160(&mut self) -> Result<H160, ERC20Error> {
		self.skip((256 - 160) / 8)?;
		self.next_h160_not_padded()
	}

	/// Returns the next H160 with no padding to 32 bytes.
	pub fn next_h160_not_padded(&mut self) -> Result<H160, ERC20Error> {
		let vec_resp = self.next_vec(20)?;
		let mut the_vec: [u8; 20] = [0; 20];
		for i in 0..20 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}

	/// Returns the next H256.
	pub fn next_h256(&mut self) -> Result<H256, ERC20Error> {
		let vec_resp = self.next_vec(32)?;
		let mut the_vec: [u8; 32] = [0; 32];
		for i in 0..32 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}

	/// Returns the next U256.
	pub fn next_u256(&mut self) -> Result<U256, ERC20Error> {
		let vec_resp = self.next_vec(32)?;
		let mut the_vec: [u8; 32] = [0; 32];
		for i in 0..32 {
			the_vec[i] = vec_resp[i];
		}
		Ok(the_vec.into())
	}
}

/// Converts H160, H256, and U256 into `Vec<u8>` which can be used to create a `Bytes`.
pub struct FixedNumberToBytes {
	data: Vec<u8>,
}

impl FixedNumberToBytes {
	/// Pushes a vector of bytes to the tail of the current byte array.
	///
	/// # Arguments
	///
	/// * `vec` - Vector with the bytes to be added.
	///
	pub fn push_vec(&mut self, vec: &Vec<u8>) {
		for it in vec {
			self.data.push(*it);
		}
	}

	/// Pushes a H160 to the tail of the current byte array.
	///
	/// # Arguments
	///
	/// * `value` - H160 to be pushed.
	///
	pub fn push_h160(&mut self, value: &H160) {
		for _ in 0..((256 - 160) / 8) {
			self.data.push(0);
		}
		self.push_h160_not_padded(value);
	}

	/// Pushes a H160 to the tail of the current byte array, with no padding to 32 bytes.
	///
	/// # Arguments
	///
	/// * `value` - H160 to be pushed.
	///
	pub fn push_h160_not_padded(&mut self, value: &H160) {
		for it in value.0.iter() {
			self.data.push(*it);
		}
	}

	/// Pushes a H256 to the tail of the current byte array.
	///
	/// # Arguments
	///
	/// * `value` - H256 to be pushed.
	///
	pub fn push_h256(&mut self, value: &H256) {
		for it in value.0.iter() {
			self.data.push(*it);
		}
	}

	/// Pushes an U256 to the tail of the current byte array.
	///
	/// # Arguments
	///
	/// * `value` - U256 to be pushed.
	///
	pub fn push_u256(&mut self, value: &U256) {
		for i in 0..(256 / 8) {
			self.data.push(value.byte(i));
		}
	}
}

impl From<Bytes> for FixedNumberToBytes {
	fn from(data: Bytes) -> Self {
		Self {
			data: data.0
		}
	}
}

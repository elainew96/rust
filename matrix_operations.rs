use std::{ops, fmt};
#[derive(PartialEq,Debug)]
pub struct Matrix<T> {
	data: Vec<T>,
	row: usize,
	col: usize
}

impl<T: Copy> Matrix<T> {
	pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
		let datas = values.to_vec();
		Matrix{
			col: col,
			row: row,
			data:datas,
		}
	}

	pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
		Matrix {
			col: col,
			row: row,
			data: Vec::new(),
		}
	}
	
	pub fn data(&self) -> &Vec<T> {
		let ref stack1 = self.data;
		&stack1
	}

	pub fn mut_data(&mut self) -> &mut Vec<T> {
		let stack2 = &mut self.data;
		stack2
	}
	
	pub fn size(&self) -> (usize, usize) {
		let (x,y) = (self.row,self.col);
		(x,y)
	}
}
impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		if self.row != rhs.row || self.col != rhs.col{
			panic!();
		}
		let mut vec = Vec::new();
		for i in 0..self.row*self.col{
			vec.push(self.data[i] + rhs.data[i]);
		}
		Matrix {
			row: self.row,
			col: self.col,
			data: vec,
		}
	}
}
impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
	type Output = Self;
	fn sub(self, rhs: Self) -> Self::Output {
		if self.row != rhs.row || self.col != rhs.col{
			panic!();
		}
		let mut vec = Vec::new();
		for i in 0..self.row*self.col{
			vec.push(self.data[i] - rhs.data[i]);
		}
		Matrix {
			row: self.row,
			col: self.col,
			data: vec,
		}
	}
}
impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy + Default> ops::Mul for Matrix<T> {
	type Output = Self;
	fn mul(self, rhs:Self) -> Self::Output {
		if self.col != rhs.row {
			panic!();
		}
		let mut vec = Vec::new();
		for i in 0..self.row{
			for j in 0..rhs.col{
				let mut sum: T=Default::default();
				for k in 0..self.col{
					let hold=self.data[i*self.col+k]*rhs.data[k*rhs.col+j];
					sum = sum + hold;
				}
				vec.push(sum);
			}
		}
		Matrix {
			row: self.row,
			col: rhs.col,
			data: vec,
		}
	}
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		let mut t = String::new();
		for i in 0..self.row{
			let mut s = String::new();
			let mut x;
			let mut y;
			for j in 0..self.col{
				x = &self.data[j+i*self.col];
				y=x.to_string();
				s.push_str(&y);
				s.push(' ');
			}
			t.push_str(&s);
			let v = t.pop();
			t.push('\n');
		}
		write!(f,"{}",t)
	}
}

// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use super::*;
pub use polkadot_primitives::{CandidateHash, Hash};

#[derive(Default, Debug)]
struct Y {
	#[allow(dead_code)]
	x: u8,
}

#[test]
fn plain() {
	error!("plain");
}

#[test]
fn wo_alias() {
	let a: i32 = 7;
	error!(target: "foo",
		"Something something {}, {b:?}, or maybe {c}",
		a,
		b = Y::default(),
		c = a
	);
}

#[test]
fn wo_unnecessary() {
	let a: i32 = 7;
	warn!(target: "bar",
		a = a,
		b = ?Y::default(),
		"fff {c}",
		c = a,
	);
}

#[test]
fn if_frequent() {
	let a: i32 = 7;
	let mut f = Freq::new();
	warn_if_frequent!(
		freq: f,
		target: "bar",
		a = a,
		b = ?Y::default(),
		"fff {c}",
		c = a,
	);
}

#[test]
fn w_candidate_hash_value_assignment() {
	let a: i32 = 7;
	info!(target: "bar",
		a = a,
		// ad-hoc value
		candidate_hash = %CandidateHash(Hash::repeat_byte(0xF0)),
		b = ?Y::default(),
		c = ?a,
		"xxx",
	);
}

#[test]
fn w_candidate_hash_from_scope() {
	let a: i32 = 7;
	let candidate_hash = CandidateHash(Hash::repeat_byte(0xF1));
	debug!(target: "bar",
		a = a,
		?candidate_hash,
		b = ?Y::default(),
		c = ?a,
		"xxx",
	);
}

#[test]
fn w_candidate_hash_aliased() {
	let a: i32 = 7;
	let c_hash = Hash::repeat_byte(0xFA);
	trace!(target: "bar",
		a = a,
		candidate_hash = ?c_hash,
		b = ?Y::default(),
		c = a,
		"xxx",
	);
}

#[test]
fn w_candidate_hash_aliased_unnecessary() {
	let a: i32 = 7;
	let candidate_hash = CandidateHash(Hash::repeat_byte(0xFA));
	info!(
		target: "bar",
		a = a,
		candidate_hash = ?candidate_hash,
		b = ?Y::default(),
		c = a,
		"xxx",
	);
}

#[test]
fn frequent_at_third_time() {
	let mut freq = Freq::new();
	let rate = 1.0;

	assert!(!freq.is_frequent(rate));
	assert!(!freq.is_frequent(rate));

	assert!(freq.is_frequent(rate));
}

#[test]
fn not_frequent_at_third_time_if_slow() {
	let mut freq = Freq::new();
	let rate = 1000.0;

	assert!(!freq.is_frequent(rate));
	assert!(!freq.is_frequent(rate));

	std::thread::sleep(std::time::Duration::from_millis(10));
	assert!(!freq.is_frequent(rate));
}

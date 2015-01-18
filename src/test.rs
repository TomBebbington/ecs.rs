use testy::Bencher;
use std::iter::range;
use std::collections::{VecMap, HashMap};
struct Position {
	x: f64,
	y: f64
}
fn get_vec() -> VecMap<Position> {
	range(0us, 100).map(|i| (i, Position {x: i as f64, y: i as f64})).collect()
}
fn get_map() -> HashMap<usize, Position> {
	range(0us, 100).map(|i| (i, Position {x: i as f64, y: i as f64})).collect()
}
#[bench]
fn bench_vec_index(b: &mut Bencher) {
	let vec = get_vec();
	b.iter(|| {
		let mut sum = 0.0f64;
		for i in 0us..100 {
			sum += vec[i].x + vec[i].y;
		}
		sum
	})
}
#[bench]
fn bench_map_index(b: &mut Bencher) {
	let map = get_map();
	b.iter(|| {
		let mut sum = 0.0f64;
		for i in 0us..100 {
			sum += map[i].x + map[i].y;
		}
		sum
	})
}
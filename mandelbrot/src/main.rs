use num::Complex;

fn complex_square_add_loop(c: Complex<f64>) {
	let mut z = Complex{ re: 0.0, im: 0.0};
	loop {
		z = z * z + c;
	}
}


/// 尝试测定`c`是否位于曼德博集中，使用最多`limit`次迭代来判定
///
/// 如果`c`不是集合成员之一，则返回`Some(i)`，其中的`i`是`c`离开以原点
/// 为中心的半径为2的圆时所需的迭代次数。如果`c`似乎是集合成员之一（确
/// 切而言是达到了迭代次数限制但仍然无法证明`c`不是成员），则返回`None
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
	let mut z = Complex { re: 0.0, img: 0.0 };
	for i in 0..limit {

} 


fn main() {
    println!("Hello, world!");
}

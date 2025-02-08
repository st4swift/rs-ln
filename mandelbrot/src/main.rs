use num::Complex;
use std::str::FromStr;


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
	let mut z = Complex { re: 0.0, im: 0.0 };
	for i in 0..limit {
		if z.norm_sqr() > 4.0 { return Some(i); }
		z = z * z + c;
	}

	None
} 

/// 把字符串`s`（形如`"400x600"`或`"1.0,0.5"`）解析成一个坐标对
///
/// 具体来说，`s`应该具有<left><sep><right>的格式，其中<sep>是由`separator`
/// 参数给出的字符，而<left>和<right>是可以被`T::from_str`解析的字符串。
/// `separator`必须是ASCII字符
///
/// 如果`s`具有正确的格式，就返回`Some<(x, y)>`；如果无法正确解析，就返回`None`

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
	match s.find(separator) {
		None => None,
		Some(index) => {
			match (T::from_str(&s[..index]), T::from_str(&s[index+1..])) {
				(Ok(x), Ok(y)) => Some((x, y)),
				_ => None
			}
		}
	}
}

#[test]
fn test_parse_pair() {
assert_eq!(parse_pair::<i32>("", ','), None);
assert_eq!(parse_pair::<i32>("10,", ','), None);
assert_eq!(parse_pair::<i32>(",10", ','), None);
assert_eq!(parse_pair::<i32>("10,20", ','), Some((10, 20)));
assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
assert_eq!(parse_pair::<f64>("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// 把一对用逗号分隔的浮点数解析为复数

fn parse_complex(s: &str) -> Option<Complex<f64>> {
	match parse_pair(s, ',') {
		Some((re, im)) => Some(Complex {re, im}),
		None => None
	}
}

#[test]
fn test_parse_complex() {
assert_eq!(parse_complex("1.25,-0.0625"),
Some(Complex { re: 1.25, im: -0.0625 }));
assert_eq!(parse_complex(",-0.0625"), None);
}



fn main() {
    println!("Hello, world!");
}

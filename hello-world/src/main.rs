// 用到crate: acitx-web, serde

// route "/" : display "hello world"
// route "/echo" : nothing
// route "/hey" : 显示输入两个u64. 并准备计算最大公约数
// route "/gcd" : caculate and display 


use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };
use serde::Deserialize;


#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hello world - actix.")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}

#[get("/hey")]
async fn manual_hello() -> impl Responder {
	HttpResponse::Ok()
	//.body("Hey there - actix.")
	.content_type("text/html")
    .body(
            r#"
                <title>GCD Calculator</title>
                <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
                </form>
            "#,
      )
}


#[derive(Deserialize)]
struct GcdParameters { n: u64, m: u64 }

#[post("/gcd")]
async fn post_gcd( form: web::Form<GcdParameters> ) ->  impl Responder {
	if form.n == 0 || form.m == 0 {
		return HttpResponse::BadRequest()
			.content_type("text/html")
			.body("Computing the GCD with zero is boring.");
	}
	HttpResponse::Ok()
		.content_type("text/html")
		.body( format!("The great common divisor of the number {} and {} is <b>{}</b>\n",
			form.n, form.m, gcd(form.n, form.m) ) )
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new( || {
		App::new()
		.service(hello)
		.service(echo)
		.service(manual_hello)
		.service(post_gcd)
		
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}	


//计算最大公约数
fn gcd(mut n: u64, mut m: u64) -> u64 {
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m = m % n;
	}
	n
}



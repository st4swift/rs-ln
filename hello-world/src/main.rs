use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder };

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("hello world - actix.")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
	HttpResponse::Ok().body(req_body)
}


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


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new( || {
		App::new()
		.service(hello)
		.service(echo)
		.route("/hey", web::get().to(manual_hello))
	})
	.bind(("127.0.0.1", 3000))?
	.run()
	.await
}	

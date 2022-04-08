use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _context: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get("/helloworld", |_, _ctx| Response::ok("Hello World!"))
        .get("/crashtheworker", |_, _ctx| panic!("CRASH"))
        .run(req, env)
        .await
}

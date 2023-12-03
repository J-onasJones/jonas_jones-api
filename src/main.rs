use warp::Filter;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let root = warp::path::end()
        .map(|| "Hello, World!");


    let routes = hello.or(root);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;

    warp::serve(routes)
        .run(([192,168,0,55], 3030))
        .await;
}

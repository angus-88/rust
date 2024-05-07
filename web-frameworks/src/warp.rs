use warp::Filter;

// Built on Hyper
pub async fn basic() {
    // https://github.com/seanmonstar/warp/blob/master/examples/routing.rs

    let routes = warp::any().map(|| "hello world");
    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

pub async fn routing() {
    // /sum/3/4 > 3 + 4 = 7
    let _sum = warp::path("sum")
        .and(warp::path::param::<u32>())
        .and(warp::path::param::<u32>())
        .and(warp::path::end())
        .map(|a, b| format!("{} + {} = {}", a, b, a + b));

    // Equivalent routes with macro
    let sum = warp::path!("sum" / u32 / u32).map(|a, b| format!("{} + {} = {}", a, b, a + b));

    let times = warp::path!(u32 / "times" / u32).map(|a, b| format!("{} * {} = {}", a, b, a * b));

    let routes = warp::get().and(sum).or(times);

    warp::serve(routes).run(([127, 0, 0, 1], 7878)).await;
}

use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn hello_world1() {
    hello_world().await;
}

fn main() {
    let future = hello_world1(); // Nothing is printed
    block_on(future); // `future` is run and "hello, world!" is printed
}

extern crate actix_web;
extern crate futures;
extern crate tokio;

use actix_web::{server, App, HttpRequest, Responder};
use futures::future::ok;
use futures::Future;
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

fn subscribe<H>(handler: H) -> impl Future<Item = (), Error = ()> + Send
where
    H: Fn(u32) -> () + Send,
{
    ok(()).and_then(move |_| {
        println!("Spawned");

        handler(100);

        ok(())
    })
}

#[derive(Clone)]
struct Store {
    // rt: Runtime,
}

impl Store {
    pub fn new() -> Self {
        Self {
            // rt: Runtime::new().unwrap(),
        }
    }

    pub fn sub<H>(&self, handler: H)
    where
        H: Fn(u32, &Self) -> () + Sync + Send + 'static,
    {
        let _self = self.clone();

        let sub = subscribe(move |num| {
            handler(num, &_self);
        });

        // self.rt.spawn(sub);

        tokio::spawn(sub);
    }
}

fn greet(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn main() {
    // let fut = ok(()).and_then(move |_| {
    //     let mut store = Store::new();

    //     store.sub(3);
    //     store.sub(2);

    //     ok(())
    // });

    // tokio::run(fut);

    server::new(|| {
        println!("Start server");

        let store = Store::new();

        store.sub(|num, _store| println!("Sub 1 {}", num));
        store.sub(|num, _store| println!("Sub 2 {}", num));

        App::new()
            .resource("/", |r| r.f(greet))
            .resource("/{name}", |r| r.f(greet))
    })
    .bind("127.0.0.1:8000")
    .expect("Can not bind to port 8000")
    .run();
}

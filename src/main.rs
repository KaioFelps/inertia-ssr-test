struct SsrClient<T> {
    pub sender: hyper::client::conn::http1::SendRequest<T>
}

fn main() {
    println!("Hello, world!");
}

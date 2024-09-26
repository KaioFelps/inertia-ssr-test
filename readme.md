# Testing Ssr server requests!
This repo is intended to be a test of an [`inertia-rust`](https://github.com/KaioFelps/inertia-rust/blob/main/src/inertia.rs) application workflow.

Lets consider that `inertia-rust::Inertia::render(...)` confirmed that there is a ssr server:
- inertia will make a request sending an InertiaPage object to the ssr client with [reqwest](https://github.com/seanmonstar/reqwest);
- the ssr server will return a `InertiaAppResponse` object;
- `inertia-rust` will use this object to render the html template and continue the workflow.

## Running

First, start the actix server that contains an mocked endpoint that will return an json response
containing the `InertiaAppResponse` object.
```bash
cargo run -p server
```

Next, just run the main binary and, if everything work, the response object will be printed
in your terminal!
```bash
cargo run
```

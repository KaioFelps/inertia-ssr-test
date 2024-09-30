# Testing Inertia SSR Requests!
This repo is intended to be a test of an [`inertia-rust`](https://github.com/KaioFelps/inertia-rust) application workflow.

Lets consider that `inertia-rust::Inertia::render(...)` confirmed that there is a ssr server:
- inertia will make a request sending an InertiaPage object to the ssr client with [reqwest](https://github.com/seanmonstar/reqwest);
- the ssr server will return a `InertiaAppResponse` object;
- `inertia-rust` will use this object to render the html template and continue the workflow.

## Running

Just run the below command on your terminal. If everything work, the response object will be printed in your terminal!
```bash
cargo run
```

The server will:
- start a node.js supervisioned process that runs an inertia ssr server;
- mock a `InertiaPage` object and makes a request to a local address at port 1000;
- compare the response with an expected response
    - before comparing, it replaces every `&quot;` by " and removes a empty comment added
    at the response for some reason idk

See also [node server readme](/nodejs-server/README.md) for some 

---

## Changelog

Before the last commit, it would use an actix web server to return the inertia app response, but as inertia
provides a node server and every adapter also uses it, the actix server was removed and then an
nodejs process supervisor has been added.
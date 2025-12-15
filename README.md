this is a demo project. I'm testing the ability to make a nextjs-like `/api/` route handler for building a backend system with js. the project is written in rust. using: MetaCall as a runtime for executing the JavaScript code users write, `axum` to power the backend system, `tokio` as an async runtime.

## TODO

- [x] simple working demo (GET Requests)
- [x] add other HTTP methods: GET/POST/DELETE/PUT
- [ ] add caching for GET routes
- [ ] add dynamic routing `api/user/[id]`
- [ ] add Rust Language
- [x] use `MetaCall` instead of deno
- [ ] add polyglot language support


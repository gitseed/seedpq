# FAQ

### Why isn't tokio_postgres included in the benches

It was giving error in the postgres server logs, due to some race condition or something. And I had no interest in getting to the bottom of it.

It's the only library that caused such issues, all the other libraries were well behaved during benchmarking.

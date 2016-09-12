# ndenev_rust_test_api
Web app using shared data structure experiment.

Test Rust web app using Nickel, experimenting with a shared global data structure between the Nickel workers and separate updater thread.

## Running

To start just use ```cargo run```:

```
macbook-pro:test-api ndenev$ cargo run --release
   Compiling test-api v0.1.0 (file:///Users/ndenev/test-api)
[ ... snipping compiler warnings ...]
2016-09-12T15:09:51 0576109000ns - DEBUG - Instantiataing new data structure
2016-09-12T15:09:51 0576161000ns - DEBUG - Copying the global data structure elements to the new one.
2016-09-12T15:09:51 0576798000ns - DEBUG - Updating global data structure to point to the new data.
2016-09-12T15:09:51 0577005000ns - DEBUG - Pausing
...
...
...
```

## Calling

Get the number of the elements in the global data structure: (1000 hardcoded in main.rs)
```
Marias-MacBook-Pro:wrk2 ndenev$ curl -s 'http://127.0.0.1:6767/stats' | jq
{
  "data_len": 1000
}
```

Get random 10 elements from the datastructure and serialize to json (10 is also hardcoded in main.rs)
```
Marias-MacBook-Pro:wrk2 ndenev$ curl -s 'http://127.0.0.1:6767/test' | jq
[
  [
    "2867413b-766b-4a3f-9e0d-1a754ee402e8",
    {
      "id": "2867413b-766b-4a3f-9e0d-1a754ee402e8",
      "data": 10654115982638504000
    }
  ],
  [
    "3e58b5cc-0cb3-46c4-9011-97079b72bd3d",
    {
      "id": "3e58b5cc-0cb3-46c4-9011-97079b72bd3d",
      "data": 13716858148837353000
    }
  ],
  [
    "2c4b5206-5ebd-486f-a07f-a298f198d02a",
    {
      "id": "2c4b5206-5ebd-486f-a07f-a298f198d02a",
      "data": 13929844130647263000
    }
  ],
  [
    "333539b6-bc22-4b34-8761-15ead57f084d",
    {
      "id": "333539b6-bc22-4b34-8761-15ead57f084d",
      "data": 13017102836244328000
    }
  ],
  [
    "bb790f12-ed02-41ac-82a4-350a66e33a54",
    {
      "id": "bb790f12-ed02-41ac-82a4-350a66e33a54",
      "data": 6595290701036234000
    }
  ],
  [
    "1cde3ace-8383-41c0-bac1-663303e71f1b",
    {
      "id": "1cde3ace-8383-41c0-bac1-663303e71f1b",
      "data": 12484309078854152000
    }
  ],
  [
    "8fd88490-cec4-4eb2-9fa2-2c376c795215",
    {
      "id": "8fd88490-cec4-4eb2-9fa2-2c376c795215",
      "data": 10860311203324891000
    }
  ],
  [
    "36dd67a7-354d-4b84-b276-0f4efaacce71",
    {
      "id": "36dd67a7-354d-4b84-b276-0f4efaacce71",
      "data": 11011174044042170000
    }
  ],
  [
    "741496d5-4a27-4fd2-b0d0-c595cabe04b0",
    {
      "id": "741496d5-4a27-4fd2-b0d0-c595cabe04b0",
      "data": 8947478692710437000
    }
  ],
  [
    "ebb1340b-23e1-49ab-b54f-718e18a22955",
    {
      "id": "ebb1340b-23e1-49ab-b54f-718e18a22955",
      "data": 12994851400796266000
    }
  ]
]
Marias-MacBook-Pro:wrk2 ndenev$
```

Run some tests:
```
Marias-MacBook-Pro:wrk2 ndenev$ ./wrk -t4 -c10 -d10 -R 1000000 http://127.0.0.1:6767/stats
Running 10s test @ http://127.0.0.1:6767/stats
  4 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.93s     2.83s    9.86s    57.94%
    Req/Sec        nan       nan   0.00      0.00%
  134020 requests in 10.01s, 19.56MB read
Requests/sec:  13390.19
Transfer/sec:      1.95MB
Marias-MacBook-Pro:wrk2 ndenev$ ./wrk -t4 -c10 -d10 -R 1000000 http://127.0.0.1:6767/test
Running 10s test @ http://127.0.0.1:6767/test
  4 threads and 10 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     5.15s     2.84s    9.94s    58.18%
    Req/Sec        nan       nan   0.00      0.00%
  59069 requests in 9.99s, 72.20MB read
Requests/sec:   5912.45
Transfer/sec:      7.23MB
Marias-MacBook-Pro:wrk2 ndenev$
```

# A minimal example reproducing a bug in Cloudflare Workers

## Bug description

When a Rust-based worker panics, it leaves the worker dangling and unable to process subsequent requests from the same client (client A), which manifests itself in consistent 500s returned by Cloudflare. Requests from another client (client B) work fine which suggests the sticky routing mechanism causes the requests sent by the client A to be routed to a dangling non-functional instance of the worker.

## Steps to reproduce

```sh
$ export CF_ACCOUNT_ID=[ACCOUNT ID]
$ wrangler publish
$ curl 
$ curl -s -o /dev/null -w "%{http_code}" https://worker-dangling-state-reproduction.[ACCOUNT NAME].workers.dev/helloworld
200
$ curl -s -o /dev/null -w "%{http_code}" https://worker-dangling-state-reproduction.[ACCOUNT NAME].workers.dev/crashtheworker
500
$ curl -s -o /dev/null -w "%{http_code}" https://worker-dangling-state-reproduction.[ACCOUNT NAME].workers.dev/helloworld
500
```

As you can see from the code (`src/lib.rs`) the `/helloworld` route cannot possibly panic, so the 500 is unwarranted.
The steps above reproduce the issue with a high degree of consistency but sometimes it may take a few attempts at the last `curl` for a 500 to be observed, so there's some non-determinism at play.

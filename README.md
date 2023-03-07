# tonic-bidi-error

Simple setup to reproduce issue

https://github.com/hyperium/tonic/issues/848

## Example

Start the server with

```
RUST_LOG=info cargo run --release
```

Run the client example with

```
cargo run --example client_end_stream --release
```

The client creates 3 streams and closes 2 of them by dropping them. This causes them
to cancel.

The third one is closed with a broken pipe because the client goes away.

```
[2023-03-07T19:00:03Z ERROR tonic_bidi_error] caught error: Status { code: Unknown, message: "error reading a body from connection: stream error received: stream no longer needed", source: Some(hyper::Error(Body, Error { kind: Reset(StreamId(1), CANCEL, Remote) })) }
[2023-03-07T19:00:03Z ERROR tonic_bidi_error] caught error: Status { code: Unknown, message: "error reading a body from connection: stream error received: stream no longer needed", source: Some(hyper::Error(Body, Error { kind: Reset(StreamId(3), CANCEL, Remote) })) }
[2023-03-07T19:00:03Z ERROR tonic_bidi_error] caught error: Status { code: Unknown, message: "error reading a body from connection: stream closed because of a broken pipe", source: Some(hyper::Error(Body, Error { kind: Io(Custom { kind: BrokenPipe, error: "stream closed because of a broken pipe" }) })) }
```

All three end up with tonic status `Unknown`.

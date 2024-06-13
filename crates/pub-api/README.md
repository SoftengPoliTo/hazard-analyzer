# Public API

`pub-api` retrieves the `Ascot` framework public `APIs`.
More specifically, it extracts [ascot-library](https://github.com/SoftengPoliTo/ascot-firmware/tree/master/src) and [ascot-axum](https://github.com/SoftengPoliTo/ascot-firmware/tree/master/ascot-axum) public `APIs`.

## Workflow

To do this, the [rustdoc-json](https://github.com/Enselic/cargo-public-api/tree/main/rustdoc-json) library is used to create the [rustdoc-json](https://rust-lang.github.io/rfcs/2963-rustdoc-json.html) documentation for `ascot-library` and `ascot-axum`, from which public `APIs` are extracted.
At the end the tool produces a `JSON` manifest that contains the following `API` types:
- `struct`  
- `enum`  
- `trait`
- `function`
- `macro`

For `struct` and `enum`, all functions directly implemented by them are shown, as well as those that are a consequence of a `trait` implementation or derivation. 

For a `trait`, all the provided (with a default implementation) and required (that need to be implemented) functions are shown. 

## Building

Use this command to build the tool:

```console
cargo build 
```

## Testing

Testing has been performed via snapshots, using [insta](https://insta.rs). Use the following command to launch the tests:

``` console
cargo insta test
```

To review the snapshots, use:

``` console
cargo insta review
```

The next command combines the previous two operations:

``` console
cargo insta test --review
```

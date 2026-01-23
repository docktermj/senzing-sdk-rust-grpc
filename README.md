# sz-sdk-rust-grpc

If you are beginning your journey with [Senzing],
please start with [Senzing Quick Start guides].

You are in the [Senzing Garage] where projects are "tinkered" on.
Although this GitHub repository may help you understand an approach to using Senzing,
it's not considered to be "production ready" and is not considered to be part of the Senzing product.
Heck, it may not even be appropriate for your application of Senzing!

## :warning: WARNING: sz-sdk-rust-grpc is still in development :warning: _

At the moment, this is "work-in-progress" with Semantic Versions of `0.n.x`.
Although it can be reviewed and commented on,
the recommendation is not to use it yet.

## Synopsis

The Senzing `sz-sdk-rust-grpc` packages provide a [Rust]
language Software Development Kit adhering to the
[sz-sdk-rust] interfaces that communicates with a [Senzing gRPC server].

## Overview

The Senzing `sz-sdk-rust-grpc` packages enable Rust programs to call Senzing library functions
across a network to a
[Senzing gRPC server](https://github.com/senzing-garage/servegrpc).

Other implementations of the [sz-sdk-rust]
interface include:

- [sz-sdk-rust-core] - for calling Senzing SDK APIs natively
- [sz-sdk-rust-mock] - for unit testing calls to the Senzing Go SDK
- [rust-sdk-abstract-factory] - An [abstract factory pattern] for switching among implementations

## Use

(TODO:)

## References

1. [Development]
1. [Errors]
1. [Examples]
1. [Package reference]

[abstract factory pattern]: https://en.wikipedia.org/wiki/Abstract_factory_pattern
[Development]: docs/development.md
[Errors]: docs/errors.md
[Examples]: docs/examples.md
[rust-sdk-abstract-factory]: https://github.com/senzing-garage/go-sdk-abstract-factory
[Rust]: https://github.com/senzing-garage/knowledge-base/blob/main/WHATIS/go.md
[Package reference]: https://pkg.go.dev/github.com/senzing-garage/sz-sdk-go-grpc
[Senzing Garage]: https://github.com/senzing-garage
[Senzing gRPC server]: https://github.com/senzing-garage/servegrpc
[Senzing Quick Start guides]: https://docs.senzing.com/quickstart/
[Senzing]: https://senzing.com/
[sz-sdk-rust-core]: https://github.com/senzing-garage/sz-sdk-rust-core
[sz-sdk-rust-mock]: https://github.com/senzing-garage/sz-sdk-rust-mock
[sz-sdk-rust]: https://github.com/senzing-garage/sz-sdk-rust

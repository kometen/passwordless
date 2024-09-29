The passwordless code is based on https://github.com/davidzr/passwordless-rust and the Bitwarden passwordless service at https://bitwarden.com/products/passwordless/.

The webframework is Rocket: https://rocket.rs/

A GET to `/pwd` will generate five passwords with default settings. A POST to `/pwd` will alter the defaults if this body is supplied

```
{"strict": true, "length": 25}
```

Password-crate: https://docs.rs/passwords/latest/passwords/index.html

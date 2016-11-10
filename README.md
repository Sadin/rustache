Rustache [![Build Status](https://travis-ci.org/rustache/rustache.svg?branch=master)](https://travis-ci.org/rustache/rustache)
====

[Rustache](https://rustache.github.io/) is a [Rust](https://www.rust-lang.org/) implementation of the [Mustache](https://mustache.github.io/) spec.

## Documentation

The different Mustache tags are documented at the [mustache(5)](http://mustache.github.com/mustache.5.html) man page.

The project's docs page is located [here](https://rustache.github.io/rustache/).

## Install

Install it through [Cargo](https://crates.io/):

```toml
rustache = "^0.0"
```

Then link it within your crate like so:

```rust
extern crate rustache;
```

## API Methods

The main forward interface that users will interact with when using Rustache are the `rustache::render_file` method and the `rustache::render_text` methods like so:

```rust
// Renders the given template string
let data = rustache::HashBuilder::new().insert("name", "your name");
let out = Cursor::new(Vec::new());
rustache::render_text("{{ name }}", data, &mut out).unwrap();
println!("{}", String::from_utf8(rv.unwrap().into_inner()).unwrap());

// Renders the given template file
let data = rustache::HashBuilder::new().insert("name", "your name");
let out = Cursor::new(Vec::new());
rustache::render_file("test_data/cmdline_test.tmpl", data, &mut out).unwrap();
println!("{}", String::from_utf8(rv.unwrap().into_inner()).unwrap());
```

## Examples

Here's an example of how to pass in data to the `render_text` method using the `HashBuilder`:

```rust
let data = HashBuilder::new()
    .insert("name", "Bob");
let out = Cursor::new(Vec::new());

rustache::render_text("{{ name }}", data, &mut out);
```

Here's an example of how to pass in data in the form of a JSON `enum` to a `render` method:

```rust
let data = json::from_str(r#"{"name": "Bob"}"#);
let out = Cursor::new(Vec::new());

rustache::render_text("{{ name }}", data, &mut out);
```

## Testing

Simply clone and run:

```bash
cargo test
```

## Roadmap

- [ ] Full Mustache spec compliance.
	- [ ] Comment and Section whitespace handling
	- [ ] Handle change of delimeters

- [ ] Thread errors through the parser and compiler:

- [ ] Fill out samples folder.

- [ ] Handle Build and Parser operations concurrently

## Contribute

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to contribute.

## License

Copyright (c) 2014 Team Rustache

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.


Inspired by https://github.com/erickt/rust-mustache:

Copyright (c) 2012 Erick Tryzelaar

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

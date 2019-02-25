# hammock

A crate to help you interact with REST APIs.

APIs usually require you to set a custom user agent and to rate limit your requests,
which I found to be pretty boilerplate in the various API-consuming programs I kept on writing.
So, I figured I'd abstract all that boilerplate into this crate.

## Using

```rust
// Set the ratelimiting settings.
// In this example, you're allowing Hammock to fetch an API endpoint once per second.
// Hammock requests will block if you make requests faster than this, until it's been one second
// since the last request.
let duration = Duration::new(1, 0);

// Set the user agent that Hammock will use when making API requests.
let user_agent = "MyApiConsumer v0.1.1 (user@example.com)";

// Actually create the Hammock object.
let mut hammock = Hammock::new(duration, user_agent).unwrap();

// Make a GET request, get the response body back as plaintext.
println!("{}", hammock.get_text("http://some.website/api/get_users")?);

// Make a GET request; get the response body back as a Serde JSON object.
println!("{:#?}", hammock.get_json("http://some.website/api/get_users?fmt=json")?);

```

## TODO

 - More customizable rate limiting
 - Authentication
 - Support for `POST`, `PUT`, etc.

## License

MIT. See the `LICENSE` file for more information.

Any contributions to this repository are assumed to be MIT unless specified otherwise.
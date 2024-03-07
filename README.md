# Problem Details Mapper

The Problem Details Mapper is a library designed to simplify error handling and response generation in Rust-based RESTful API applications. It allows you to map errors to Problem Details as defined by the [RFC 7807](https://datatracker.ietf.org/doc/html/rfc7807) specification, enabling consistent and standardized error responses across your API.

## Features
* **Error Mapping**: Easily map Rust errors to Problem Details objects, providing detailed information about encountered errors.
* **RFC 7807 Compliance**: Ensures that error responses adhere to the RFC 7807 standard, promoting interoperability and consistency in API responses. Provided by the [ProblemDetails](https://crates.io/crates/problem_details) crate.
* **Customizable**: Customize the Problem Details response format and content to suit the requirements of your application.
* **Extensible**: Extend functionality with custom error types and mappings tailored to your application's needs.

## Getting Started
To integrate the Rust Problem Details Mapper into your project, add the following line to your Cargo.toml file:

```toml
[dependencies]
problem-details-mapper = "0.1.0"
```

Then, import the library into your Rust code:
```rust
use problem_details_mapper::{
    IntoProblemDetails,
    ProblemDetails,
    ProblemDetailsBuilder,
    ProblemDetailsMapper
};
```

## Usage
Here's a simple example demonstrating how to use the Problem Details Mapper:
```rust
use problem_details_mapper::{
    IntoProblemDetails,
    ProblemDetails,
    ProblemDetailsBuilder,
    ProblemDetailsMapper
};

#[derive(Debug)]
struct TestError;

impl TestError {
    pub fn new() -> Self {
        Self
    }
}

impl std::fmt::Display for TestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for TestError {}

impl IntoProblemDetails for TestError {
    fn into_problemdetails(&self) -> ProblemDetails {
        ProblemDetailsBuilder::build_bad_request(
            Some("test"),
            None::<String>)
    }
}

fn main() {
    // Setup the mapper settings
    ProblemDetailsMapper::setup(|options| {
        // will map every error given to a http status code 500.
        options.map_std_err();

        // will register the test error its own mapping
        // provided by IntoProblemDetails.
        options.map::<TestError>();
    });

    // Map an error to a Problem Details response
    let error = TestError::new();
    let problem_details = ProblemDetailsMapper::map(Box::new(error));

    // Output the Problem Details response
    println!("{:?}", problem_details);
}
```

## Contributing
Contributions to the Problem Details Mapper are welcome! If you encounter any issues, have ideas for improvements, or would like to contribute new features, please feel free to open an issue or submit a pull request on the repository.

## License
This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments
* Inspired by [Hellang.Middleware.ProblemDetails](https://www.nuget.org/packages/Hellang.Middleware.ProblemDetails) nuget package for C# for Problem Details Mapping in ASP.net HTTP APIs.
use problem_details_mapper::ProblemDetailsMapper;

fn main() {
    // Setup the mapper settings
    ProblemDetailsMapper::setup(|options| {
        // will map every error given to a http status code 500.
        options.map_std_err();
    }).unwrap();

    // Map an error to a Problem Details response
    let error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
    let problem_details = ProblemDetailsMapper::map(Box::new(error));

    // Output the Problem Details response
    println!("{:?}", problem_details);
}
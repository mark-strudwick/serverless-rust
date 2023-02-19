# Serverless Rust
A HTTP service for AWS Lambda, backed by any MySQL compatible database. Currently, the chosen DB provider is PlanetScale, although this can easily be changed by providing a different SQL connection in the environment variables. 

# Local testing
The application can be run locally with `cargo lambda watch`. Make sure that a .ENV file exists with a `DATABASE_URL`.

To check that the service is running and reachable, make the following HTTP request `GET http://127.0.0.1:9000/lambda-url/backend`.

To create a person in the DB, run the following HTTP request
```
POST http://127.0.0.1:9000/lambda-url/backend/person
{
  "name": "Kobe"
}
```

# Deploy
Github Actions will automatically deploy the application to AWS, by creating a Lambda, and APIGW. The resource template can be found in the `infrastructure` folder, where there is an AWS CDK stack.

# Benchmark
Run the command `drill --benchmark benchmark/benchmark.yml --stats` to see benchmark stats against a locally running application.
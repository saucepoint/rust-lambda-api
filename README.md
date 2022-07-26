## rust-lambda-api

🦀 Create HTTP APIs with Rust, hosted on AWS Lambda (serverless compute) 🦀

### *Lightweight, fast, and preorganized*

The template includes examples for routing, query parameters, request bodies, and response serialization, which are absent from *cargo-lambda*

All of the AWS infrastructure is created & handled with Terraform. What are you waiting for, anon?

---

# Setup & Dependencies

1. [Configure AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-quickstart.html)
2. Install [Rust](https://www.rust-lang.org/tools/install) & [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda)
    * `cargo install cargo-lambda`

2. [Install terraform](https://learn.hashicorp.com/tutorials/terraform/install-cli?in=terraform/aws-get-started)

2. Prepare an initial `.zip` for Terraform to upload to AWS
    ```bash
    cd lambda-api/
    cargo lambda build --output-format zip
    cd ..
    ```
2. Use terraform to spin up AWS infrastructure
    ```
    cd terraform/
    terraform init
    terraform apply
    // read the changes reported by terraform
    // type yes and hit Enter
    // Successful infra deployment will return:
    // Apply complete! Resources: 3 added, 0 changed, 0 destroyed.
    ```
    *Note:* The initial creation of the Lambda will take up to 1 min to process. If you attempt a `cargo lambda deploy` shortly after resource-creation, it will fail if the Lambda has an in-progress update

# Configuration
### Adding Routes:

1. Create a new file such as `lambda-api/src/api/foo.rs`
    - Add GET/POST handlers, Request/Response structs, similar to [src/api/hello.rs](https://github.com/saucepoint/rust-lambda-api/blob/db213e32e5c5e0fc4dd5b3b5cdf14047ff6ddaa9/lambda-api/src/api/hello.rs)

2. Register the route in `lambda-api/src/main.rs` similar to [this](https://github.com/saucepoint/rust-lambda-api/blob/1b3ccfea94e0378512a98bce56d7ef3a0f843715/lambda-api/src/main.rs#L18-L25)
    ```rust
    "/foo" => {
        match method {
            Method::POST => api::foo::post(event),
            Method::GET => api::foo::get(event),
            _ => api::errors::handle_405(),
        }
    }
    ```

### Rename the function:

1. `lambda-api/Cargo.toml`
    - name = ~~"lambda-api"~~ --> NEW_FUNCTION_NAME
2. `terraform/main.tf`
    - function_name = ~~"lambda-api"~~ --> NEW_FUNCTION_NAME
    - filename = ~~"../lambda-api/target/lambda/lambda-api/bootstrap.zip"~~ --> "../lambda-api/target/lambda/NEW_FUNCTION_NAME/bootstrap.zip"

3. Apply changes
    ```bash
    cd lambda-api/
    cargo lambda build
    cd ../terraform/
    terraform apply
    ```

You can use terraform or AWS console to attach additional infrastructure


# Local Development

In a new terminal:
```bash
cd lambda-api/

cargo lambda watch
```

Testing:

The base URL will be *127.0.0.1:9000/lambda-url/lambda-api*
```
GET 127.0.0.1:9000/lambda-url/lambda-api/hello?name=saucepoint&number=100

POST 127.0.0.1:9000/lambda-url/lambda-api/hello
// with raw JSON body:
{
    "name": "saucepoint",
    "number": 100
}
```

*Note*: when deployed to Lambda, your route will not have `/lambda-url/lambda-api` prefix:

*https://RANDOM_HASH.lambda-url.REGION.on.aws/hello*


# Deploying

Please see [cargo-lambda](https://github.com/cargo-lambda/cargo-lambda) for additional flags (such as environment variables)

This is my preferred deployment call:

*Get your IAM Role's ARN from the AWS web console*
```bash
cargo lambda build --release && cargo lambda deploy --enable-function-url --iam-role arn:aws:iam::<AWS_ACCOUNT_NUMBER>:role/rust-lambda-api 
```

---

### *Disclaimer*
This template repo is intended for hobbyists and experiments. The following bits will need to be modified & enhanced before it can be considered production-ready:

🚩 Tests - this repo totally lacks automated tests

🚩  Modularized Terraform - the current files will collide with resources of the same name. The files should be better organized & more configurable such that it can tap into existing infrastructure (VPCs)

🚩  AWS API Gateway Trigger - Typical Lambda REST APIs sit behind API Gateway. It didn't stop me from getting this prototype functional, so I didn't really bother

🚩  API authentication - this repo offers no examples around auth-required APIs

Feel free to submit PRs!

---

I'm tinkering outside of my 9-5, with plans to launch *something, eventually*

Find me on twitter [@saucepoint](https://twitter.com/saucepoint)

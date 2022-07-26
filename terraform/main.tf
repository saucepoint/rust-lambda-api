terraform {
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  required_version = ">= 1.2.0"
}

provider "aws" {
  region  = "us-east-2"
}

# -----------------------------------------------------------------------------
# IAM Role for the Lambda Function
# -----------------------------------------------------------------------------
resource "aws_iam_role" "rust-lambda-api" {
  name = "rust-lambda-api"
  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "lambda.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF
}

resource "aws_iam_policy_attachment" "attach-basiclambda" {
  name = "attach-basiclambda"
  roles = ["${aws_iam_role.rust-lambda-api.name}"]
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

# ------------------------------------------------------------
# Lambda Function
# ------------------------------------------------------------
resource "aws_lambda_function" "lambda-api" {
  function_name = "lambda-api"
  role = aws_iam_role.rust-lambda-api.arn
  handler = "bootstrap"
  runtime = "provided.al2"
  filename = "../lambda-api/target/lambda/lambda-api/bootstrap.zip"
  timeout = 30  # 30 second timeout for Lambda invokations
}

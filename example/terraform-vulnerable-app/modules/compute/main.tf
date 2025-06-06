# Compute resources - Lambda, EC2, Load Balancers

resource "aws_lambda_function" "data_processor" {
  filename         = "processor.zip"
  function_name    = "${var.project_name}-data-processor"
  role            = aws_iam_role.lambda_role.arn
  handler         = "index.handler"
  runtime         = "python3.6"
  timeout         = 300
  memory_size     = 512

  environment {
    variables = {
      DB_HOST     = var.database_endpoint
      DB_PASSWORD = var.admin_password
      API_KEY     = var.api_key
      BUCKET_NAME = var.storage_bucket
      DEBUG_MODE  = "true"
    }
  }

  tags = {
    Name        = "${var.project_name}-lambda"
    Environment = var.environment
    Purpose     = "data-processing"
  }
}

resource "aws_iam_role" "lambda_role" {
  name = "${var.project_name}-lambda-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "lambda_basic" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  role       = aws_iam_role.lambda_role.name
}

resource "aws_iam_role_policy_attachment" "lambda_vpc" {
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaVPCAccessExecutionRole"
  role       = aws_iam_role.lambda_role.name
}

resource "aws_ebs_volume" "app_data" {
  availability_zone = data.aws_availability_zones.available.names[0]
  size              = 100
  type              = "gp2"
  encrypted         = false

  tags = {
    Name        = "${var.project_name}-data-volume"
    Environment = var.environment
    Purpose     = "application-data"
  }
}

resource "aws_cloudtrail" "audit" {
  name           = "${var.project_name}-audit-trail"
  s3_bucket_name = var.storage_bucket
  enable_logging = false

  event_selector {
    read_write_type           = "All"
    include_management_events = true
    data_resource {
      type   = "AWS::S3::Object"
      values = ["arn:aws:s3:::${var.storage_bucket}/*"]
    }
  }

  tags = {
    Name        = "${var.project_name}-audit"
    Environment = var.environment
    Purpose     = "audit-logging"
  }
}

resource "aws_lb" "main" {
  name               = "${var.project_name}-alb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = var.security_group_ids
  subnets            = var.subnet_ids

  enable_deletion_protection = false

  tags = {
    Name        = "${var.project_name}-load-balancer"
    Environment = var.environment
  }
}

data "aws_availability_zones" "available" {
  state = "available"
}
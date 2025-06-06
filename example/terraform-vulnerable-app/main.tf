# Terraform Vulnerable Infrastructure Example
# This file contains intentional security vulnerabilities for testing purposes

terraform {
  required_version = ">= 0.12"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
  }
}

provider "aws" {
  region = var.aws_region
  # VULNERABILITY: Hardcoded credentials
  access_key = "AKIAIOSFODNN7EXAMPLE"
  secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
}

variable "aws_region" {
  description = "AWS region"
  type        = string
  default     = "us-west-2"
  # VULNERABILITY: No validation for region input
}

variable "admin_password" {
  description = "Database admin password"
  type        = string
  # VULNERABILITY: Not marked as sensitive
  default = "admin123"
}

# VULNERABILITY: S3 bucket with public access
resource "aws_s3_bucket" "vulnerable_bucket" {
  bucket = "my-vulnerable-bucket-${random_id.bucket_suffix.hex}"
}

resource "aws_s3_bucket_public_access_block" "vulnerable_bucket_pab" {
  bucket = aws_s3_bucket.vulnerable_bucket.id

  # VULNERABILITY: Allowing public access
  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_versioning" "vulnerable_bucket_versioning" {
  bucket = aws_s3_bucket.vulnerable_bucket.id
  versioning_configuration {
    # VULNERABILITY: Versioning disabled
    status = "Disabled"
  }
}

resource "random_id" "bucket_suffix" {
  byte_length = 8
}

# VULNERABILITY: RDS instance with multiple security issues
resource "aws_db_instance" "vulnerable_db" {
  allocated_storage    = 10
  engine               = "mysql"
  engine_version       = "5.7"
  instance_class       = "db.t3.micro"
  db_name              = "vulnerabledb"
  username             = "admin"
  password             = var.admin_password
  parameter_group_name = "default.mysql5.7"
  
  # VULNERABILITY: Publicly accessible database
  publicly_accessible = true
  
  # VULNERABILITY: No backup retention
  backup_retention_period = 0
  
  # VULNERABILITY: Skip final snapshot
  skip_final_snapshot = true
  
  # VULNERABILITY: No deletion protection
  deletion_protection = false
  
  # VULNERABILITY: No encryption
  storage_encrypted = false
  
  vpc_security_group_ids = [aws_security_group.vulnerable_db_sg.id]
}

# VULNERABILITY: Overly permissive security group
resource "aws_security_group" "vulnerable_db_sg" {
  name        = "vulnerable-db-sg"
  description = "Security group for vulnerable database"

  ingress {
    description = "MySQL from anywhere"
    from_port   = 3306
    to_port     = 3306
    protocol    = "tcp"
    # VULNERABILITY: Open to the internet
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    description = "All traffic"
    # VULNERABILITY: All ports open
    from_port   = 0
    to_port     = 65535
    # VULNERABILITY: All protocols allowed
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
}

# VULNERABILITY: IAM policy with wildcard permissions
resource "aws_iam_policy" "vulnerable_policy" {
  name        = "VulnerablePolicy"
  path        = "/"
  description = "A vulnerable IAM policy"

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        # VULNERABILITY: Allow all actions
        Action = "*"
        Effect = "Allow"
        # VULNERABILITY: On all resources
        Resource = "*"
        # VULNERABILITY: For any principal
        Principal = "*"
      },
    ]
  })
}

# VULNERABILITY: Lambda function with overly broad permissions
resource "aws_lambda_function" "vulnerable_lambda" {
  # VULNERABILITY: Dynamic filename without validation
  filename      = var.lambda_filename
  function_name = "vulnerable_function"
  role          = aws_iam_role.vulnerable_lambda_role.arn
  handler       = "index.handler"
  
  # VULNERABILITY: Potentially outdated runtime
  runtime = "python3.8"
  
  # VULNERABILITY: Source code hash from variable
  source_code_hash = var.lambda_source_hash
}

variable "lambda_filename" {
  description = "Lambda deployment package filename"
  type        = string
  # VULNERABILITY: No validation on file path
}

variable "lambda_source_hash" {
  description = "Lambda source code hash"
  type        = string
  # VULNERABILITY: External control over source hash
}

resource "aws_iam_role" "vulnerable_lambda_role" {
  name = "vulnerable_lambda_role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = "sts:AssumeRole"
        Effect = "Allow"
        Principal = {
          Service = "lambda.amazonaws.com"
        }
      },
    ]
  })
}

resource "aws_iam_role_policy_attachment" "vulnerable_lambda_policy" {
  role       = aws_iam_role.vulnerable_lambda_role.name
  # VULNERABILITY: Overly broad managed policy
  policy_arn = "arn:aws:iam::aws:policy/PowerUserAccess"
}

# VULNERABILITY: EBS volume without encryption
resource "aws_ebs_volume" "vulnerable_volume" {
  availability_zone = "${var.aws_region}a"
  size              = 40
  
  # VULNERABILITY: No encryption
  encrypted = false
}

# VULNERABILITY: CloudTrail with logging disabled
resource "aws_cloudtrail" "vulnerable_trail" {
  name           = "vulnerable-trail"
  s3_bucket_name = aws_s3_bucket.vulnerable_bucket.bucket
  
  # VULNERABILITY: Logging disabled
  enable_logging = false
}

# VULNERABILITY: Resource with force destroy enabled
resource "aws_s3_bucket" "force_destroy_bucket" {
  bucket = "force-destroy-bucket-${random_id.bucket_suffix.hex}"
  
  # VULNERABILITY: Can be force destroyed with data
  force_destroy = true
}

locals {
  # VULNERABILITY: Sensitive data in locals
  database_credentials = {
    username = "admin"
    password = "supersecret123"
    api_key  = "sk-1234567890abcdef"
  }
}

output "database_endpoint" {
  description = "RDS instance endpoint"
  value       = aws_db_instance.vulnerable_db.endpoint
  # VULNERABILITY: Sensitive information in output without sensitive flag
}

output "admin_credentials" {
  description = "Database admin credentials"
  value       = local.database_credentials
  # VULNERABILITY: Exposing credentials in output
}
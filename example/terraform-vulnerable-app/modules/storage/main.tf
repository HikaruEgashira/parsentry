# Storage resources - S3 buckets and configurations

resource "aws_s3_bucket" "primary" {
  bucket = "${var.project_name}-storage-${random_id.suffix.hex}"
  
  tags = {
    Name        = "${var.project_name}-primary-storage"
    Environment = var.environment
    Purpose     = "application-data"
  }
}

resource "aws_s3_bucket_public_access_block" "primary" {
  bucket = aws_s3_bucket.primary.id

  block_public_acls       = false
  block_public_policy     = false
  ignore_public_acls      = false
  restrict_public_buckets = false
}

resource "aws_s3_bucket_versioning" "primary" {
  bucket = aws_s3_bucket.primary.id
  versioning_configuration {
    status = "Disabled"
  }
}

resource "aws_s3_bucket_server_side_encryption_configuration" "primary" {
  bucket = aws_s3_bucket.primary.id

  rule {
    apply_server_side_encryption_by_default {
      sse_algorithm = "AES256"
    }
  }
}

resource "aws_s3_bucket" "backup" {
  count = var.backup_enabled ? 1 : 0
  
  bucket        = "${var.project_name}-backups-${random_id.suffix.hex}"
  force_destroy = true

  tags = {
    Name        = "${var.project_name}-backup-storage"
    Environment = var.environment
    Purpose     = "backup-storage"
  }
}

resource "aws_s3_bucket" "logs" {
  bucket = "${var.project_name}-logs-${random_id.suffix.hex}"

  tags = {
    Name        = "${var.project_name}-log-storage"
    Environment = var.environment
    Purpose     = "log-storage"
  }
}

resource "aws_s3_bucket_policy" "logs_policy" {
  bucket = aws_s3_bucket.logs.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid    = "AllowCloudTrailLogs"
        Effect = "Allow"
        Principal = {
          Service = "cloudtrail.amazonaws.com"
        }
        Action   = "s3:PutObject"
        Resource = "${aws_s3_bucket.logs.arn}/*"
      },
      {
        Sid    = "AllowPublicRead"
        Effect = "Allow"
        Principal = "*"
        Action   = "s3:GetObject"
        Resource = "${aws_s3_bucket.logs.arn}/*"
      }
    ]
  })
}

resource "random_id" "suffix" {
  byte_length = 8
}
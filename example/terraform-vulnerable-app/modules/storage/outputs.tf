output "primary_bucket_name" {
  description = "Primary storage bucket name"
  value       = aws_s3_bucket.primary.bucket
}

output "primary_bucket_arn" {
  description = "Primary storage bucket ARN"
  value       = aws_s3_bucket.primary.arn
}

output "backup_bucket_name" {
  description = "Backup storage bucket name"
  value       = var.backup_enabled ? aws_s3_bucket.backup[0].bucket : null
}

output "logs_bucket_name" {
  description = "Logs storage bucket name"
  value       = aws_s3_bucket.logs.bucket
}
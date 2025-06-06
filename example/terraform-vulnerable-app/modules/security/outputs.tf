output "app_sg_id" {
  description = "Application security group ID"
  value       = aws_security_group.app_server.id
}

output "database_sg_id" {
  description = "Database security group ID"
  value       = aws_security_group.database.id
}

output "cache_sg_id" {
  description = "Cache security group ID"
  value       = aws_security_group.cache.id
}

output "app_role_arn" {
  description = "Application IAM role ARN"
  value       = aws_iam_role.app_role.arn
}
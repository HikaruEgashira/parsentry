output "endpoint" {
  description = "Database endpoint"
  value       = aws_db_instance.main.endpoint
}

output "port" {
  description = "Database port"
  value       = aws_db_instance.main.port
}

output "cache_endpoint" {
  description = "Cache endpoint"
  value       = aws_elasticache_cluster.main.cache_nodes[0].address
}
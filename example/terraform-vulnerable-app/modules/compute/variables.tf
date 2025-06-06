variable "environment" {
  description = "Environment name"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for compute resources"
  type        = list(string)
}

variable "security_group_ids" {
  description = "List of security group IDs"
  type        = list(string)
}

variable "database_endpoint" {
  description = "Database endpoint for Lambda environment"
  type        = string
}

variable "admin_password" {
  description = "Database password for Lambda environment"
  type        = string
  sensitive   = true
}

variable "api_key" {
  description = "External API key for Lambda environment"
  type        = string
  sensitive   = true
}

variable "storage_bucket" {
  description = "S3 bucket name for Lambda environment"
  type        = string
}
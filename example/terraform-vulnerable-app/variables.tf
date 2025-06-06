# Variables for the e-commerce platform

variable "aws_region" {
  description = "AWS region for deployment"
  type        = string
  default     = "us-west-2"
}

variable "environment" {
  description = "Environment name"
  type        = string
  default     = "production"
}

variable "project_name" {
  description = "Project name for resource naming"
  type        = string
  default     = "ecommerce"
}

variable "admin_password" {
  description = "Database admin password"
  type        = string
  default     = "admin123"
}

variable "allowed_cidr" {
  description = "Allowed CIDR for admin access"
  type        = string
  default     = "0.0.0.0/0"
}

variable "backup_enabled" {
  description = "Enable backup for storage"
  type        = bool
  default     = false
}

variable "backup_retention" {
  description = "Backup retention period in days"
  type        = number
  default     = 0
}

variable "api_key" {
  description = "API key for external services"
  type        = string
  default     = "sk-1234567890abcdef"
}
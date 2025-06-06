variable "environment" {
  description = "Environment name"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "subnet_ids" {
  description = "List of subnet IDs for database"
  type        = list(string)
}

variable "security_group_ids" {
  description = "List of security group IDs"
  type        = list(string)
}

variable "admin_password" {
  description = "Database admin password"
  type        = string
}

variable "backup_retention" {
  description = "Backup retention period in days"
  type        = number
  default     = 0
}
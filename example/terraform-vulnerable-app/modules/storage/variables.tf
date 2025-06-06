variable "environment" {
  description = "Environment name"
  type        = string
}

variable "project_name" {
  description = "Project name"
  type        = string
}

variable "backup_enabled" {
  description = "Enable backup storage"
  type        = bool
  default     = false
}
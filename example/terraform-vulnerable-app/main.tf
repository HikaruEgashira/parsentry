# E-commerce Platform Infrastructure
# Main configuration file

terraform {
  required_version = ">= 0.12"
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.0"
    }
    random = {
      source  = "hashicorp/random"
      version = "~> 3.0"
    }
  }
}

provider "aws" {
  region     = var.aws_region
  access_key = "AKIAIOSFODNN7EXAMPLE"
  secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
}

# Networking infrastructure
module "networking" {
  source = "./modules/networking"
  
  aws_region  = var.aws_region
  environment = var.environment
  project_name = var.project_name
}

# Security groups and policies
module "security" {
  source = "./modules/security"
  
  vpc_id       = module.networking.vpc_id
  environment  = var.environment
  project_name = var.project_name
  allowed_cidr = var.allowed_cidr
}

# Storage resources
module "storage" {
  source = "./modules/storage"
  
  environment   = var.environment
  project_name  = var.project_name
  backup_enabled = var.backup_enabled
}

# Database resources
module "database" {
  source = "./modules/database"
  
  environment         = var.environment
  project_name        = var.project_name
  subnet_ids          = module.networking.private_subnet_ids
  security_group_ids  = [module.security.database_sg_id]
  admin_password      = var.admin_password
  backup_retention    = var.backup_retention
}

# Compute resources
module "compute" {
  source = "./modules/compute"
  
  environment        = var.environment
  project_name       = var.project_name
  subnet_ids         = module.networking.public_subnet_ids
  security_group_ids = [module.security.app_sg_id]
  database_endpoint  = module.database.endpoint
  storage_bucket     = module.storage.primary_bucket_name
  admin_password     = var.admin_password
  api_key            = var.api_key
}

# Global resources
resource "random_id" "project_suffix" {
  byte_length = 4
}

# Outputs
output "application_url" {
  description = "Application load balancer URL"
  value       = module.compute.load_balancer_dns
}

output "database_endpoint" {
  description = "Database connection endpoint"
  value       = module.database.endpoint
}

output "storage_bucket" {
  description = "Primary storage bucket name"
  value       = module.storage.primary_bucket_name
}

output "backup_bucket" {
  description = "Backup storage bucket name"
  value       = module.storage.backup_bucket_name
}
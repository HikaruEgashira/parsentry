# Database resources

resource "aws_db_subnet_group" "main" {
  name       = "${var.project_name}-db-subnet-group"
  subnet_ids = var.subnet_ids

  tags = {
    Name        = "${var.project_name}-db-subnet-group"
    Environment = var.environment
  }
}

resource "aws_db_instance" "main" {
  allocated_storage               = 20
  storage_type                   = "gp2"
  engine                         = "mysql"
  engine_version                 = "5.7"
  instance_class                 = "db.t3.micro"
  db_name                        = "${var.project_name}db"
  username                       = "dbadmin"
  password                       = var.admin_password
  parameter_group_name           = "default.mysql5.7"
  db_subnet_group_name           = aws_db_subnet_group.main.name
  vpc_security_group_ids         = var.security_group_ids
  publicly_accessible            = true
  backup_retention_period        = var.backup_retention
  backup_window                 = "03:00-04:00"
  maintenance_window            = "sun:04:00-sun:05:00"
  skip_final_snapshot           = true
  deletion_protection           = false
  storage_encrypted             = false
  monitoring_interval           = 0
  performance_insights_enabled  = false
  copy_tags_to_snapshot         = true

  tags = {
    Name        = "${var.project_name}-database"
    Environment = var.environment
    Application = var.project_name
  }
}

resource "aws_elasticache_subnet_group" "main" {
  name       = "${var.project_name}-cache-subnet"
  subnet_ids = var.subnet_ids
}

resource "aws_elasticache_cluster" "main" {
  cluster_id           = "${var.project_name}-cache"
  engine               = "redis"
  node_type            = "cache.t3.micro"
  num_cache_nodes      = 1
  parameter_group_name = "default.redis6.x"
  port                 = 6379
  subnet_group_name    = aws_elasticache_subnet_group.main.name
  security_group_ids   = var.security_group_ids

  tags = {
    Name        = "${var.project_name}-cache"
    Environment = var.environment
    Purpose     = "session-cache"
  }
}
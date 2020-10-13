provider "aws" {
  region = "eu-west-3"
}

resource "aws_db_instance" "default" {
  allocated_storage    = 20
  engine               = "postgres"
  engine_version       = "11.5"
  instance_class       = "db.t2.micro"
  name                 = var.db_name
  username             = var.db_user
  password             = var.db_password
  deletion_protection  = true
  tags = {
      Name = "rustipe-db"
  }
}


resource "aws_security_group" "sg_web" {
  name = "security-group-web-server"
  ingress {
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  ingress {
    from_port   = 443
    to_port     = 443
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]
  }

  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }
  tags = {
      Name = "security-group-web-server"
  }
}

resource "aws_security_group" "ssh_access" {
    name = "security-group-ssh_access"
    ingress {
        from_port   = 22
        to_port     = 22
        protocol    = "tcp"
        description = "ssh"
        cidr_blocks = ["0.0.0.0/0"]
    }
    tags = {
        Name = "security-group-ssh_access"
    }
}

resource "aws_s3_bucket" "photo_bucket_name" {
  bucket = var.photo_bucket_name
  acl    = "public-read"
  tags = {
      Name = var.photo_bucket_name
  }
}

resource "aws_iam_role" "rustipe_ec2_role" {
  name = "rustipe_ec2_role"

  assume_role_policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": "sts:AssumeRole",
      "Principal": {
        "Service": "ec2.amazonaws.com"
      },
      "Effect": "Allow",
      "Sid": ""
    }
  ]
}
EOF

  tags = {
      Name = "rustipe-web-server-role"
  }
}

resource "aws_iam_instance_profile" "rustipe_ec2_profile" {
  name = "rustipe_ec2_profile"
  role = aws_iam_role.rustipe_ec2_role.name
}

resource "aws_iam_role_policy" "rustipe_bucket_policy" {
  name = "rustipe_bucket_policy"
  role = aws_iam_role.rustipe_ec2_role.id

  policy = <<EOF
{
  "Version": "2012-10-17",
  "Statement": [
    {
      "Action": [
        "s3:*"
      ],
      "Effect": "Allow",
      "Resource":"arn:aws:s3:::${var.photo_bucket_name}"
    }
  ]
}
EOF
    #   "Resource":"arn:aws:s3:::${var.photo_bucket_name}",
}

resource "aws_key_pair" "rustipe_deployer_key" {
  key_name   = "rustipe_deployer_key"
  public_key = var.public_key
}

resource "aws_instance" "web_server" {
  ami           = "ami-0de12f76efe134f2f"
  instance_type = "t2.micro"
  iam_instance_profile = "${aws_iam_instance_profile.rustipe_ec2_profile.name}"
  vpc_security_group_ids = [aws_security_group.sg_web.id, aws_security_group.ssh_access.id]
  tags = {
      Name = "Rustipe web server"
  }
  key_name = aws_key_pair.rustipe_deployer_key.key_name
}

resource "local_file" "ansible_inventory" {
  content = templatefile("${path.module}/templates/hosts.tpl",
    {
      web_ip = aws_instance.web_server.*.public_ip
    }
  )
  filename = "../ansible/inventory/hosts.cfg"
}

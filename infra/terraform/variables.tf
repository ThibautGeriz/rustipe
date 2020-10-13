variable "photo_bucket_name" {
  type    = string
  default = "rustipe-photos"
}

variable "public_key" {
    type    = string
    default = "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABAQDAQp+NjaX7DhkQZGJ6EeRTBP7ApDWDIqm/crjb9LoxWV6pWKd8iudwdv88G8jnbCHEuD5gbvSLFHqP88Dvkfu0/AiAU0oiDQhO9mKCRe4zQ3rIJpKSWY01BevXDoG9FnjWbSlnpdR5uMXqR2EqdgzGeoK3QFrQDQalajrl79ERRbbbYQ6bvc6CxTzeW0gDd6FIlzs61Eb8cK69WXQuFKoiIp/zDPsfbHK+7NXJi5B7OKYt+yBfLnMNzoyDoyTljYZLlN78Om0ibUNqxV3A5r5zOMMDYrB0RPPaRTwF6ZUIRaRUeokhFl+HAcx/K0wQbTTBEeSX1CFQ1wVf7qYD8cmr thibautgery@Thibauts-MacBook-Pro.local"
}

variable "db_name" {
    type    = string
    default = "rustipe"
}

variable "db_user" {
    type    = string
    default = "web_app"
}

variable "db_password" {
    type    = string
}

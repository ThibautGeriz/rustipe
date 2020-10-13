[web]
%{ for ip in web_ip ~}
${ip}
%{ endfor ~}

[all:vars]
ansible_ssh_user = ec2-user

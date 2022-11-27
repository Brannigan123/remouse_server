# Remouse Server
Receives requests over https to emulate keyboard and mouse events on X11 systems.

Uses Linux PAM for authentication.

## Configuring SSL
------------------
Requires key.pem and cert.pem files to be provided.

For a self-signing option, create the key.pem and cert.pem use the command and fill in your own subject

> $ openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"

To remove the password prompt, run the below command then copy nopass.pem to key.pem

> $ openssl rsa -in key.pem -out nopass.pem
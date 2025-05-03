{
  services.openssh = {
    enable = true;

    settings = {
      PasswordAuthentication = false;
      KbdInteractiveAuthentication = false;
      PermitRootLogin = "no";
    };
  };

  users.users.alex.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMIfTLm+mUFmMfydIkJioTceAtrN9BEXkbbhqGFjBUeL alex@jiruo"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIM3zQ1YpAnr2PYx8WMHXXhg6tCk/YH8NF+sVMxntH/WR alex@marulk"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMpZkrT0NGEbVnXwVRtuAngLlIQQkEqDHypDzDOdyOKa alex@qemu"
  ];
}

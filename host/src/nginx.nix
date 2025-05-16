let
  domain = "tust.adelille.rocks";
in {
  services.nginx = {
    enable = true;

    recommendedProxySettings = true;
    recommendedTlsSettings = true;
    recommendedOptimisation = true;
    recommendedGzipSettings = true;

    virtualHosts."${domain}" = {
      forceSSL = true;
      enableACME = true;

      locations."/" = {
        proxyPass = "http://localhost:3000";
      };
    };
  };

  security.acme = {
    acceptTerms = true;
    defaults.email = "adelille@student.42.fr";
  };

  networking.firewall = {
    enable = true;

    allowedTCPPorts = [80 443];
  };
}

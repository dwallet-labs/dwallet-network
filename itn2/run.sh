#!/usr/bin/env bash

./ika-arm64-mac validator config-env --ika-package-id 0x278ece343c16131fb72d41bf07ac2c233af8545c07625a6655ba3d6e92ebc8e0 \
--ika-system-package-id 0x9eaf6efc4d96a44bb46de7b4dcbfef2d826bbaef0925ead81c1bc3f0c9798ab8 \
--ika-system-object-id 0xb8fbfb48b0d5597a5840761a04c50c0ac1c76d75539649fe32fbf853380180ce

./ika-arm64-mac validator make-validator-info "Ika Validator" "Ika Validator" "https://cdn.prod.website-files.com/67161f6a7534fbf38021d666/67accc5d35ca2f9f0e927ac1_ika_logo_round.svg" "https://ika.xyz/" "ika-validator1.itn2.ika-network.net" 1000 0xcc38aca540158c12fa9fd35a339a3ea9125f1052e1dfc18a98a3e53f7dacbbc2


./ika validator become-candidate ./validator.info


# Validator ID: 0x3334a27afdc66dbd54aabe8d945da2e504465ef42a11744d030c288b38faa6a6
  #Validator Cap ID: 0x8962e93314dfa3ede2bb7e799b2591f6893b19349356c1892f0f0a7d9110f44c

./ika validator join-committee --validator-cap-id 0x8962e93314dfa3ede2bb7e799b2591f6893b19349356c1892f0f0a7d9110f44c

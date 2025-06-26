sui keytool generate ed25519 "m/44'/784'/0'/0'/0'" word24 --json


MNEMONIC="where point drill enemy ostrich mutual trim empower bless absent van tortoise shiver soft near toilet armed midnight best airport afraid pen traffic lamp"
sui keytool import "$MNEMONIC" ed25519 "m/44'/784'/0'/0'/0'"

## ╭─────────────────┬──────────────────────────────────────────────────────────────────────╮
   #│ alias           │  gracious-opal                                                       │
   #│ suiAddress      │  0x234b74348914b6fe0ea4955cd129b0f43da52c6a4d54a5382412b997683de5aa  │
   #│ publicBase64Key │  AGy0PYc3LtdU+9vybmDrMAtESvC1mLzW6bEY9A+gy8ep                        │
   #│ keyScheme       │  ed25519                                                             │
   #│ flag            │  0                                                                   │
   #│ peerId          │  6cb43d87372ed754fbdbf26e60eb300b444af0b598bcd6e9b118f40fa0cbc7a9    │
   #╰─────────────────┴──────────────────────────────────────────────────────────────────────╯

sui keytool update-alias gracious-opal notifier


gcloud secrets versions add itn2-ika-fullnode-keys --data-file="notifier.key" --project sui-validators

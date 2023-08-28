import { Chain, AssetList } from '@chain-registry/types';
export const nibiruChian: Chain = {
    "chain_id": "nibiru-itn-1",
    "chain_name": "nibiru-itn-1",
    "status": "live",
    "network_type": "testnet",
    "pretty_name": "nibiru test net",
    "apis": {
      "rpc": [
          {
              "address": "https://rpc.itn-1.nibiru.fi",
          }
      ],
      "rest": [
          {
              "address": "https://lcd.itn-1.nibiru.fi"
          }
      ],
      "grpc": []
  },
    "staking": {
      "staking_tokens": [
          {
              "denom": "unibi"
          }
      ]
  },
  "slip44": 118,
  "key_algos": [
    "secp256k1"
  ],
  "bech32_prefix": "nibi",
    "fees": {
      "fee_tokens": [
          {
              "denom": "unibi",
              "fixed_min_gas_price": 0,
              "low_gas_price":0.05,
              "average_gas_price": 0.125,
              "high_gas_price": 0.2
          }
      ]
  },
};

export const nibiruAssets: AssetList = {
  "chain_name": "nibiru-itn-1",
  "assets": [
      {
          "description": "The native token of nibiru",
          "denom_units": [
              {
                  "denom": "unibi",
                  "exponent": 0,
                  "aliases": []
              },
              {
                  "denom": "NIBI",
                  "exponent": 6,
                  "aliases": []
              }
          ],
          "base": "unibi",
          "name": "NIBI",
          "display": "nibi",
          "symbol": "NIBI",
          "logo_URIs": {
            "svg": "https://app.nibiru.fi/static/media/iconNIBI.568d89b7c290f1b0b2f5c092b2f0dd32.svg"
          },
          "keywords": [
              "dex",
              "staking"
          ]
      },
      {
          "denom_units": [
              {
                  "denom": "unusd",
                  "exponent": 0
              },
              {
                  "denom": "NUSD",
                  "exponent": 6
              }
          ],
          "base": "unusd",
          "name": "NUSD",
          "display": "NUSD",
          "symbol": "NUSD",
          "coingecko_id": "ion",
          "logo_URIs": {
            "svg": "https://app.nibiru.fi/static/media/iconNUSD.30192bfda0480cbed13d8345361ef811.svg"
          },
          "keywords": [
              "memecoin"
          ]
      }
  ]
};
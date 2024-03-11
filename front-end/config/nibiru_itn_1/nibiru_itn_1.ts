import { Chain, AssetList } from '@chain-registry/types';
export const nibiruChian: Chain = {
    "chain_id": "cataclysm-1",
    "chain_name": "cataclysm-1",
    "status": "live",
    "network_type": "mainnet",
    "pretty_name": "nibiru mainnet net",
    "apis": {
      "rpc": [
          {
              "address": "http://149.50.101.148:3090/submit_pfb",
          }
      ],
      "rest": [
          {
              "address": "https://lcd.nibiru.fi:443"
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
  "chain_name": "cataclysm-1",
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
            "svg": "https://app.nibiru.fi/assets/coloredNUSD-ce40c602.svg"
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
            "svg": "https://app.nibiru.fi/assets/coloredTether-5bdcd470.svg"
          },
          "keywords": [
              "memecoin"
          ]
      }
  ]
};
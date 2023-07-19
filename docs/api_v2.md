# Findora Scanner V2 API Spec


* `/api/v2/ex/evm/:hash`
* Response:
```json
{
	"code": 200,
	"data": {
		"block": "e1ed231539cba0e40b4d595bde42eb0717a0e3634ae8368acddf293f4a02d00f",
		"evm_tx_hash": "0xe102840e773316fa6f39fd9687ab2ee831ee398ca59074b75a99d6da07b382c9",
		"from": "0xc7671515ef928ce0ee3a1920e2ea120442efb1ea",
		"height": 2799317,
		"timestamp": 1661763710,
		"to": "0x4754c883946253e66f7232546ab6fb70c488b26a",
		"tx_hash": "5ed3a7d62b17668537bff6bb1659b03cd583079dea068422b1eef45361d59de0",
		"ty": 1,
		"value": {
			"function": {
				"Ethereum": {
					"Transact": {
						"action": {
							"Call": "0x4754c883946253e66f7232546ab6fb70c488b26a"
						},
						"gas_limit": "0xf4240",
						"gas_price": "0xba43b7400",
						"input": [68, 26, 62, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 201, 139, 118, 0, 0],
						"nonce": "0x46e",
						"signature": {
							"r": "0x7d13c3e94b314c752b45c2c3096800be554ca94d0bd41972569d8c3f5c1006fd",
							"s": "0x0e36cec88e4935934ee4fc74503beab383bd93dce3b9eae2c3e3e9c04c66ce79",
							"v": 4339
						},
						"value": "0x0"
					}
				}
			}
		}
	},
	"message": ""
}
```
* `/api/v2/txs/evm`
* Response:
```json
{
    "code": 200,
    "data": {
        "page": 1,
        "page_size": 10,
        "total": 1,
        "txs": [
            {
                "block": "e1ed231539cba0e40b4d595bde42eb0717a0e3634ae8368acddf293f4a02d00f",
                "evm_tx_hash": "0xe102840e773316fa6f39fd9687ab2ee831ee398ca59074b75a99d6da07b382c9",
                "from": "0xc7671515ef928ce0ee3a1920e2ea120442efb1ea",
                "height": 2799317,
                "timestamp": 1661763710,
                "to": "0x4754c883946253e66f7232546ab6fb70c488b26a",
                "tx_hash": "5ed3a7d62b17668537bff6bb1659b03cd583079dea068422b1eef45361d59de0",
                "ty": 1,
                "value": {
                    "function": {
                        "Ethereum": {
                            "Transact": {
                                "action": {
                                    "Call": "0x4754c883946253e66f7232546ab6fb70c488b26a"
                                },
                                "gas_limit": "0xf4240",
                                "gas_price": "0xba43b7400",
                                "input": [68, 26, 62, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 134, 201, 139, 118, 0, 0],                             
                                "nonce": "0x46e",
                                "signature": {
                                    "r": "0x7d13c3e94b314c752b45c2c3096800be554ca94d0bd41972569d8c3f5c1006fd",
                                    "s": "0x0e36cec88e4935934ee4fc74503beab383bd93dce3b9eae2c3e3e9c04c66ce79",
                                    "v": 4339
                                },
                                "value": "0x0"
                            }
                        }
                    }
                }
            }
        ]
    },
    "message": ""
}
```
* `/api/v2/tx/delegation/:hash`
* Response:
```json
{
	"code": 200,
	"data": {
		"amount": 1000000,
		"block_hash": "af9c552f9a24283e8493823f4b188f63d3b3efb5c7a23cec3d31c13eaa038024",
		"from": "fra18fnyetvs2kc035xz78kyfcygmej8pk5h2kwczy03w6uewdphzfxsk74dym",
		"height": 2799430,
		"new_validator": "",
		"timestamp": 1661765633,
		"tx_hash": "5ca21da32f029318e811db17f9240659c98e86169bd42c1b7901afff71b425b3",
		"validator": "9E6717392EFDCFA101E33449A7C2A238251315B1",
		"value": {
			"Delegation": {
				"body": {
					"amount": 1000000,
					"new_validator": null,
					"nonce": [
						[234, 72, 153, 142, 247, 120, 11, 7], 39960
					],
					"validator": "9E6717392EFDCFA101E33449A7C2A238251315B1"
				},
				"pubkey": "OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=",
				"signature": "HEh--QNjRhigsXtOddRFaJISvHc-in86C8xDp3RW2icffPzOJlU7OS_nHsKpVO6wHlXC6vnzsEHm6ju2Qmi1DQ==",
				"v_signature": null
			}
		}
	},
	"message": ""
}
```
* `/api/v2/tx/undelegation/:hash`
* Response
```json
{
	"code": 200,
	"data": {
		"amount": 1000000,
		"block_hash": "cf66fcf9c09fa13c4838db8dbdcb3d22950d575fd65a1cf0025489e356ff8555",
		"from": "fra18fnyetvs2kc035xz78kyfcygmej8pk5h2kwczy03w6uewdphzfxsk74dym",
		"height": 2799433,
		"new_delegator": "Vp_Ph2OobZuAOQ7vS7uXyjYcToAVoDGpo-hX55yAKLQ=",
		"target_validator": "9E6717392EFDCFA101E33449A7C2A238251315B1",
		"timestamp": 1661765687,
		"tx_hash": "91001c320aa13cef240f00b1cd941a429b72de9f089ccd301111998aa55d6562",
		"value": {
			"UnDelegation": {
				"body": {
					"nonce": [
						[153, 102, 51, 31, 1, 178, 206, 47], 39961
					],
					"pu": {
						"am": 1000000,
						"new_delegator_id": "Vp_Ph2OobZuAOQ7vS7uXyjYcToAVoDGpo-hX55yAKLQ=",
						"target_validator": [158, 103, 23, 57, 46, 253, 207, 161, 1, 227, 52, 73, 167, 194, 162, 56, 37, 19, 21, 177]
					}
				},
				"pubkey": "OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=",
				"signature": "uvIqs2b8YXCTgHcGZKYYCcEwBCFl2cUg5xn5fXzBXNPimQNBIe6xf8naxBH_2Mr9sWFsgI-WgRIPGju61T9XBA=="
			}
		}
	},
	"message": ""
}
```
* `/v2/tx/claim/:hash`
* Response
```json
{
	"code": 200,
	"data": {
		"amount": 20000,
		"block_hash": "951992a07625251f2cbf7805c063dd182af3fd06641cc1400ebea46a0d5834c1",
		"from": "fra1xczgryuz65as77gf8d5f07xd0wetd8qpm5hvgqkfgc60gxdjpmkshnq9ys",
		"height": 2799448,
		"timestamp": 1661765944,
		"tx_hash": "114710d4828472bc3e2da38089b70efe92d47f25b8b0439b918de227a9cbf392",
		"value": {
			"Claim": {
				"body": {
					"amount": 20000,
					"nonce": [
						[216, 83, 96, 111, 84, 31, 175, 85], 39962
					]
				},
				"pubkey": "NgSBk4LVOw95CTtol_jNe7K2nAHdLsQCyUY09BmyDu0=",
				"signature": "2DMtKlq-VM17sLI7xJVql5lXR1v4W-wYDZ8Zlf8gVu09GWmgGjPQuaD9BrtX_0dYQ4bRgiMe9SrbK7qJOI6qAg=="
			}
		}
	},
	"message": ""
}
```
* `/v2/tx/n2e/:hash`
* Response:
```json
{
	"code": 200,
	"data": {
		"amount": "1000000",
		"asset": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
		"block_hash": "e1ed231539cba0e40b4d595bde42eb0717a0e3634ae8368acddf293f4a02d00f",
		"from": "OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=",
		"height": 2799317,
		"timestamp": 1661763710,
		"to": "0x6f6050950cfa13f612388cd793242458acca4aa7",
		"tx_hash": "e5fb78efd8dac071a7d8e5e9217971fe552302f74aad811106a314f1c3029ccd",
		"value": {
			"ConvertAccount": {
				"nonce": [
					[133, 28, 181, 142, 101, 188, 209, 200], 39950
				],
				"receiver": {
					"Ethereum": "0x6f6050950cfa13f612388cd793242458acca4aa7"
				},
				"signer": "OmZMrZBVsPjQwvHsROCI3mRw2pdVnYER8Xa5lzQ3Ek0=",
				"value": "1000000"
			}
		}
	},
	"message": ""
}
```
* `/v2/asset/define/:asset`
* Response:
```json
{
	"code": 200,
	"data": {
		"asset": "dx8LUysx7w3sgKDN8voPw3HVqFU2eW7ZkxwE9HPATjM=",
		"block": "cf457fd6bbd404761cf837ae56e74c2191f5f3662040ec2dd7a4f4004c1004a4",
		"decimal": 6,
		"height": 2798685,
		"issuer": "fra1xczgryuz65as77gf8d5f07xd0wetd8qpm5hvgqkfgc60gxdjpmkshnq9ys",
		"max_units": "",
		"timestamp": 1661752937,
		"tx": "f8ff841a53603e40b5628e9df7d662a72cc9d60c9035521d6dc530d35f2679f0",
		"value": {
			"DefineAsset": {
				"body": {
					"asset": {
						"asset_rules": {
							"decimals": 6,
							"max_units": "",
							"transfer_multisig_rules": null,
							"transferable": true,
							"updatable": false
						},
						"code": {
							"val": [119, 31, 11, 83, 43, 49, 239, 13, 236, 128, 160, 205, 242, 250, 15, 195, 113, 213, 168, 85, 54, 121, 110, 217, 147, 28, 4, 244, 115, 192, 78, 51]
						},
						"issuer": {
							"key": "NgSBk4LVOw95CTtol_jNe7K2nAHdLsQCyUY09BmyDu0="
						},
						"memo": "ARP"
					}
				},
				"pubkey": {
					"key": "NgSBk4LVOw95CTtol_jNe7K2nAHdLsQCyUY09BmyDu0="
				},
				"signature": "pPXgtGw8D6JmYYQ1fQlV5TJhhHI1T-yatNJnMXAzP8wMBd6mJjz9-RCM3-1YQ2vHm1DFBIpUf0ZLW_yp_Ea8DQ=="
			}
		}
	},
	"message": ""
}
```
* `/api/v2/asset/issue/:asset`
* Response:
```json
{
  "code": 200,
  "data": {
    "asset": "dx8LUysx7w3sgKDN8voPw3HVqFU2eW7ZkxwE9HPATjM=",
    "block": "3294db9641e0653ffed2f8368eaf52a0da40bfee5c43322368249b4ae00a5f2d",
    "height": 2798689,
    "issuer": "fra1xczgryuz65as77gf8d5f07xd0wetd8qpm5hvgqkfgc60gxdjpmkshnq9ys",
    "timestamp": 1661753006,
    "tx": "fe5958ff051f6c397869320452b4308cc171228e59154b806236ab514ed75582",
    "value": {
      "IssueAsset": {
        "body": {
          "code": {
            "val": [119, 31, 11, 83, 43, 49, 239, 13, 236, 128, 160, 205, 242, 250, 15, 195, 113, 213, 168, 85, 54, 121, 110, 217, 147, 28, 4, 244, 115, 192, 78, 51]
          },
          "num_outputs": 1,
          "records": [
            [{
              "id": null,
              "record": {
                "amount": {
                  "Confidential": ["BA5weX5FN4bul50T2jcsOCZImgh8qFAMcmW1XqP1IRg=", "7mRIrNV2dxXTzGdFYlBFmD2y0RZul5k_EZVdX8mNKFk="]
                },
                "asset_type": {
                  "NonConfidential": [119, 31, 11, 83, 43, 49, 239, 13, 236, 128, 160, 205, 242, 250, 15, 195, 113, 213, 168, 85, 54, 121, 110, 217, 147, 28, 4, 244, 115, 192, 78, 51]
                },
                "public_key": "NgSBk4LVOw95CTtol_jNe7K2nAHdLsQCyUY09BmyDu0="
              }
            }, {
              "blind_share": "VglwyHFCYL7pg87itoDyvtcOJ_9X4rovtelydidMJ2I=",
              "lock": {
                "ciphertext": "ZyqqjNYByqs=",
                "ephemeral_public_key": "-uj1yfgazboYNA4kvPSGWYT2m9_UFc35LJt3MJ-4ZSI="
              }
            }]
          ],
          "seq_num": 39902
        },
        "pubkey": {
          "key": "NgSBk4LVOw95CTtol_jNe7K2nAHdLsQCyUY09BmyDu0="
        },
        "signature": "DkydH0ONCMm5yiUKmWHEFZTLF3IfpYyyTkv-drg8ExvU6MyMxtEyr99PuWWManMJw3hpaopVATVpPN44w0WiCg=="
      }
    }
  },
  "message": ""
}
```
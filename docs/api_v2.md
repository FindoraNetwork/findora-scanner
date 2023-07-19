# Findora Scanner V2 API Spec

Get evm tx by hash:
`/api/v2/evm/tx/:hash`
Response:
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
`/api/v2/evm/txs`
Response:
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
* `/api/v2/delegation/tx/:hash`
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
* `/api/v2/undelegation/tx/:hash`
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
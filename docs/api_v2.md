# Findora Scanner V2 API Spec

Get evm tx by hash:
`/api/v2/tx/evm/:hash`
Response:
```json
{
	"code": 200,
	"data": [{
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
	}],
	"message": ""
}
```
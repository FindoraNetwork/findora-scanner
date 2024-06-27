# Findora Explorer API Spec

## Transaction
* [1.1 根据哈希获取交易](#1.1)
* [1.2 获取交易列表](#1.2)
* [1.3 交易分布](#1.3)

## Block
* [2.1 根据哈希获取区块](#2.1)
* [2.2 获取区块列表](#2.2)

## Chain
* [3.1 统计交易量](#3.1)
* [3.2 地址总量](#3.2)

## Asset
* [4.1 获取资产信息](#4.1)

## 其他
* [5.1 获取FRA市场信息](#5.1)
* [5.2 获取FRA单价](#5.2)

<h3 id="1.1">1.1 根据交易哈希获取交易</h3>

* `GET /api/tx`

| 参数   | 类型     | 必传 | 说明   |
|------|--------|----|------|
| hash | string | Y  | 交易哈希 |


* Request: `/api/tx?hash=07a6d26221d92156180b46b9aa7bd31b00e943b01e180cdd80797c9f4f8e997d`
* Response:
```json
{
	"tx_hash": "07a6d26221d92156180b46b9aa7bd31b00e943b01e180cdd80797c9f4f8e997d",
	"evm_tx_hash": "",
	"block_hash": "2F2494696B4B6E5F7C659882E7E760079191A2C443C5296EF3473DF94748C55F",
	"height": 5812495,
	"timestamp": 1719293627,
	"ty": 1,
	"code": 0,
	"log": "",
	"origin": "ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4YWE2NDYiLCJnYXNfcHJpY2UiOiIweDI1NDBiZTQwMCIsImdhc19saW1pdCI6IjB4MjQ5ZjAiLCJhY3Rpb24iOnsiQ2FsbCI6IjB4YjUyZWYyOTI2NTFmODY5MjA1M2FjM2QzOTkyMWE0M2ZlMTM1MzNjOCJ9LCJ2YWx1ZSI6IjB4MCIsImlucHV0IjpbMTk4LDY0LDExNyw0NSwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwyLDEyLDI0NiwxOTMsNDMsOCwxNzIsMTkwLDEyMCw4NCw3Niw3OSw1MSwyNSwxNjcsNzMsMTc2LDk2LDIzLDEyOCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwxNiwwXSwic2lnbmF0dXJlIjp7InYiOjQzNDAsInIiOiIweDFiMTdmYjcyODQ0OTZkZjRiZDY0Yzg3M2I2NzcwMTEyMzFlMjQ5NDBjNzIyYTNlNmE4MmRmNDlkODQ4NTE1ZWUiLCJzIjoiMHg3NTU4NDVhODViNmIxODAyM2QzZTcwNjIyYzE4YzRkNDdkZDJmMTMxNDE1ZWYzZDRmNmFmZmU5NmFiNGNiMjY2In19fX19",
	"result": {
		"code": 0,
		"codespace": "",
		"data": "eyJDYWxsIjp7ImV4aXRfcmVhc29uIjp7IlN1Y2NlZWQiOiJTdG9wcGVkIn0sInZhbHVlIjpbXSwidXNlZF9nYXMiOiIweDlkNmIiLCJsb2dzIjpbXX19",
		"events": [{
			"attributes": [{
				"key": "c2VuZGVy",
				"value": "MHhkM2UwNzViZDAzMTQ5MDk3ZmExMzViMThhODc1NmI1MDI0YTQ1ZTVl"
			}, {
				"key": "dG8=",
				"value": "MHhiNTJlZjI5MjY1MWY4NjkyMDUzYWMzZDM5OTIxYTQzZmUxMzUzM2M4"
			}, {
				"key": "Y29udHJhY3RfYWRkcmVzcw==",
				"value": "MHgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAw"
			}, {
				"key": "dHJhbnNhY3Rpb25faGFzaA==",
				"value": "MHg2OGEzNzhhNTQ1OTY3NzI3NTRhNzdjOTE3YjU1Mzk3MWE3YzgxOTA0OTU4YjJiZGQxZmY4NGIwMmRhYjRkNGQ2"
			}, {
				"key": "cmVhc29u",
				"value": "U3VjY2VlZChTdG9wcGVkKQ=="
			}],
			"type": "ethereum_TransactionExecuted"
		}],
		"gasUsed": "40299",
		"gasWanted": "150000",
		"info": "",
		"log": ""
	},
	"value": {
		"function": {
			"Ethereum": {
				"Transact": {
					"action": {
						"Call": "0xb52ef292651f8692053ac3d39921a43fe13533c8"
					},
					"from": "0xd3e075bd03149097fa135b18a8756b5024a45e5e",
					"gas_limit": "0x249f0",
					"gas_price": "0x2540be400",
					"input": [198, 64, 117, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 12, 246, 193, 43, 8, 172, 190, 120, 84, 76, 79, 51, 25, 167, 73, 176, 96, 23, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0],
					"nonce": "0xaa646",
					"signature": {
						"r": "0x1b17fb7284496df4bd64c873b677011231e24940c722a3e6a82df49d848515ee",
						"s": "0x755845a85b6b18023d3e70622c18c4d47dd2f131415ef3d4f6affe96ab4cb266",
						"v": 4340
					},
					"value": "0x0"
				}
			}
		}
	}
}
```

<h3 id="1.2">1.2 获取交易列表</h3>

| 参数         | 类型     | 必传 | 说明         |
|------------|--------|----|------------|
| from       | string | N  | 发送者        |
| to         | string | N  | 接收者        |
| ty         | number | N  | 类型         |
| start_time | number | N  | 开始时间戳      |
| end_time   | number | N  | 结束时间戳      |
| page       | number | N  | 页码，缺省值为1   |
| page_size  | number | N  | 页大小，缺省值为10 |

* Request: `/api/txs?page=2&page_size=2`
* Response:
```json
{
	"total": 2652825,
	"page": 2,
	"page_size": 2,
	"data": [{
		"tx_hash": "37bfd9eb1a899c117329e7b1573f02930412dca7a1186192c8752327f719f57d",
		"evm_tx_hash": "",
		"block_hash": "DE3AA9E720AF2D81ABF839AF404A7E445B1944C5739595B8288D13518EA869A8",
		"height": 5812537,
		"timestamp": 1719294543,
		"ty": 1,
		"code": 0,
		"log": "",
		"origin": "ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4YWE2NTEiLCJnYXNfcHJpY2UiOiIweDI1NDBiZTQwMCIsImdhc19saW1pdCI6IjB4MjQ5ZjAiLCJhY3Rpb24iOnsiQ2FsbCI6IjB4YjUyZWYyOTI2NTFmODY5MjA1M2FjM2QzOTkyMWE0M2ZlMTM1MzNjOCJ9LCJ2YWx1ZSI6IjB4MCIsImlucHV0IjpbMTk4LDY0LDExNyw0NSwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwxNiwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDYsNTAsMTg2LDE2Miw5OCwxNTMsMjAxLDE1MSw0NiwyMTIsMjE3LDE3NSwyNTAsNjMsMjA4LDg3LDE2NywzNCw4MiwyNTVdLCJzaWduYXR1cmUiOnsidiI6NDMzOSwiciI6IjB4NTBkOTA1N2ViMWE0NDJhNmI2YjMwY2I5ZDA4NzY0NWU3YTc4MmZhMTQ4YjRhM2RlNDM4YjA3Y2UxYjczNzdkZiIsInMiOiIweDFlYThjYTliNGIxOWI5YmM5ZjI1NTMxOTMyNGZkZWQyNzk0YTY4ZGQzMjNlZGRmMTY2OWZiN2QzOTMxYzg3YzEifX19fX0=",
		"result": {
			"code": 0,
			"codespace": "",
			"data": "eyJDYWxsIjp7ImV4aXRfcmVhc29uIjp7IlN1Y2NlZWQiOiJTdG9wcGVkIn0sInZhbHVlIjpbXSwidXNlZF9nYXMiOiIweDlkNmIiLCJsb2dzIjpbXX19",
			"events": [{
				"attributes": [{
					"key": "c2VuZGVy",
					"value": "MHhkM2UwNzViZDAzMTQ5MDk3ZmExMzViMThhODc1NmI1MDI0YTQ1ZTVl"
				}, {
					"key": "dG8=",
					"value": "MHhiNTJlZjI5MjY1MWY4NjkyMDUzYWMzZDM5OTIxYTQzZmUxMzUzM2M4"
				}, {
					"key": "Y29udHJhY3RfYWRkcmVzcw==",
					"value": "MHgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAw"
				}, {
					"key": "dHJhbnNhY3Rpb25faGFzaA==",
					"value": "MHg4ZWI1YTU4MDc3NDBmOTYzNjE4NTczNTRkYWRjMjBjZTYyM2Y4MWE2ZmNjYWZhZWQ2MDhlZmNjZjk5Y2U4YjNm"
				}, {
					"key": "cmVhc29u",
					"value": "U3VjY2VlZChTdG9wcGVkKQ=="
				}],
				"type": "ethereum_TransactionExecuted"
			}],
			"gasUsed": "40299",
			"gasWanted": "150000",
			"info": "",
			"log": ""
		},
		"value": {
			"function": {
				"Ethereum": {
					"Transact": {
						"action": {
							"Call": "0xb52ef292651f8692053ac3d39921a43fe13533c8"
						},
						"from": "0xd3e075bd03149097fa135b18a8756b5024a45e5e",
						"gas_limit": "0x249f0",
						"gas_price": "0x2540be400",
						"input": [198, 64, 117, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 50, 186, 162, 98, 153, 201, 151, 46, 212, 217, 175, 250, 63, 208, 87, 167, 34, 82, 255],
						"nonce": "0xaa651",
						"signature": {
							"r": "0x50d9057eb1a442a6b6b30cb9d087645e7a782fa148b4a3de438b07ce1b7377df",
							"s": "0x1ea8ca9b4b19b9bc9f255319324fded2794a68dd323eddf1669fb7d3931c87c1",
							"v": 4339
						},
						"value": "0x0"
					}
				}
			}
		}
	}, {
		"tx_hash": "a48d94fc3266ccbc4cfea37e9ca6fa863a6c7a28e13f662c9636e48ad4211120",
		"evm_tx_hash": "",
		"block_hash": "8BDD56B7BB11F4125F1CB3B3F28AC88E93518905A1415C5B593F798F0F42F8F8",
		"height": 5812536,
		"timestamp": 1719294526,
		"ty": 1,
		"code": 0,
		"log": "",
		"origin": "ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4YWE2NTAiLCJnYXNfcHJpY2UiOiIweDI1NDBiZTQwMCIsImdhc19saW1pdCI6IjB4MjQ5ZjAiLCJhY3Rpb24iOnsiQ2FsbCI6IjB4YjUyZWYyOTI2NTFmODY5MjA1M2FjM2QzOTkyMWE0M2ZlMTM1MzNjOCJ9LCJ2YWx1ZSI6IjB4MCIsImlucHV0IjpbMTk4LDY0LDExNyw0NSwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwxNiwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDE0NywyMzcsMjUwLDQ5LDIxNSwxNzIsMTA1LDE1MywxNTgsMTUwLDc3LDE3MiwxNTYsMzcsMjA1LDEwMCwyLDE5OSw5MywxNzldLCJzaWduYXR1cmUiOnsidiI6NDM0MCwiciI6IjB4MDQxMWMyZjFmM2E4NjQ1MDIxMDY1MWFlZmVjNzk5Zjk4NDllMjZlZmIyYWVmYzYyNDY4OTA5NGI2YWVlNzAyOSIsInMiOiIweDQ4OTgyZTA4YTM3NzAyZjAwNGEwZGUwYTY3OWZkMTNkMzA1YTRkMWE3ZjNjMmM4OTE3MzUyOWMxZWU3NTNmNzIifX19fX0=",
		"result": {
			"code": 0,
			"codespace": "",
			"data": "eyJDYWxsIjp7ImV4aXRfcmVhc29uIjp7IlN1Y2NlZWQiOiJTdG9wcGVkIn0sInZhbHVlIjpbXSwidXNlZF9nYXMiOiIweDlkNmIiLCJsb2dzIjpbXX19",
			"events": [{
				"attributes": [{
					"key": "c2VuZGVy",
					"value": "MHhkM2UwNzViZDAzMTQ5MDk3ZmExMzViMThhODc1NmI1MDI0YTQ1ZTVl"
				}, {
					"key": "dG8=",
					"value": "MHhiNTJlZjI5MjY1MWY4NjkyMDUzYWMzZDM5OTIxYTQzZmUxMzUzM2M4"
				}, {
					"key": "Y29udHJhY3RfYWRkcmVzcw==",
					"value": "MHgwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAw"
				}, {
					"key": "dHJhbnNhY3Rpb25faGFzaA==",
					"value": "MHhjMTBhNDU4YTY0NGM4YTc1MWYwYTVlZmEwZWQxOTZlYzI4OTRkOTAxOTlhNzc1M2U1ODU4NTQ4NjdjM2RkOGU2"
				}, {
					"key": "cmVhc29u",
					"value": "U3VjY2VlZChTdG9wcGVkKQ=="
				}],
				"type": "ethereum_TransactionExecuted"
			}],
			"gasUsed": "40299",
			"gasWanted": "150000",
			"info": "",
			"log": ""
		},
		"value": {
			"function": {
				"Ethereum": {
					"Transact": {
						"action": {
							"Call": "0xb52ef292651f8692053ac3d39921a43fe13533c8"
						},
						"from": "0xd3e075bd03149097fa135b18a8756b5024a45e5e",
						"gas_limit": "0x249f0",
						"gas_price": "0x2540be400",
						"input": [198, 64, 117, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 237, 250, 49, 215, 172, 105, 153, 158, 150, 77, 172, 156, 37, 205, 100, 2, 199, 93, 179],
						"nonce": "0xaa650",
						"signature": {
							"r": "0x0411c2f1f3a86450210651aefec799f9849e26efb2aefc624689094b6aee7029",
							"s": "0x48982e08a37702f004a0de0a679fd13d305a4d1a7f3c2c89173529c1ee753f72",
							"v": 4340
						},
						"value": "0x0"
					}
				}
			}
		}
	}]
}
```

<h3 id="1.3">1.3 交易分布</h3>

* `GET /api/txs/distribute`
* 无参数
* Request: `/api/txs/distribute`
* Response:
```json
{
  "transparent": 131087,
  "privacy": 2810,
  "prism": 10090,
  "evm_compatible": 2518950
}
```


<h3 id="2.1">2.1 根据哈希获取区块</h3>

* `GET /api/block`


| 参数   | 类型     | 必传 | 说明   |
|------|--------|----|------|
| hash | string | Y  | 区块哈希 |

* Request: `/api/block?hash=BFB5378DAB2973940EF1AB455B02EF43FC4825784DC9217E1A0C5D51E698A28F`
* Response:
```json
{
	"block_hash": "BFB5378DAB2973940EF1AB455B02EF43FC4825784DC9217E1A0C5D51E698A28F",
	"block_num": 5812496,
	"app_hash": "9ACEE49C35993BED80BD2C087DFE389B12D3D34ACB0A9DA42F15802DB8A3D697",
	"proposer": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7",
	"num_txs": 0,
	"block_size": 7726,
	"block_id": {
		"hash": "BFB5378DAB2973940EF1AB455B02EF43FC4825784DC9217E1A0C5D51E698A28F",
		"parts": {
			"total": "1",
			"hash": "CBBE643E51A03E85FFF447A32ECB519F94B91B459C71AD616E26B79FD1EB91AA"
		}
	},
	"block_header": {
		"version": {
			"block": "10",
			"app": "0"
		},
		"chain_id": "chain-qILMsV",
		"height": "5812496",
		"time": "2024-06-25T05:34:13.402347647Z",
		"last_block_id": {
			"hash": "2F2494696B4B6E5F7C659882E7E760079191A2C443C5296EF3473DF94748C55F",
			"parts": {
				"total": "1",
				"hash": "8EE34394BE6392B6E5139D1FB7DE3CA240F2677EFCEEB7F4A155B652A92D9A62"
			}
		},
		"last_commit_hash": "70C004A11E37A57C14941E58B97D627E565D615382F2639328CD6583958C9018",
		"data_hash": "",
		"validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
		"next_validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
		"consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
		"app_hash": "9ACEE49C35993BED80BD2C087DFE389B12D3D34ACB0A9DA42F15802DB8A3D697",
		"last_results_hash": "352E083368129F73731739BBD04F7964DD3CA9FB4665BC703C305CEEFE5B9A69",
		"evidence_hash": "",
		"proposer_address": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7"
	}
}
```

<h3 id="2.2">2.2 区块列表</h3>

* `GET /api/blocks`


| 参数        | 类型     | 必传 | 说明         |
|-----------|--------|----|------------|
| page      | number | N  | 页码，缺省值为1   |
| page_size | number | N  | 页大小，缺省值为10 |

* Request: `/api/blocks?page=1&page_size=2`
* Response:
```json
{
	"total": 5812561,
	"page": 1,
	"page_size": 2,
	"data": [{
		"block_hash": "3D088AFEE34E9164C06A85DF7C2F0433645C7875A127B820115303404809B3B6",
		"block_num": 5812561,
		"app_hash": "C42510F6D4B9BBA3BE08E19EF3E36A1E31701AC9A510A314EB5E2F2DCA032180",
		"proposer": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7",
		"num_txs": 1,
		"block_size": 8339,
		"block_id": {
			"hash": "3D088AFEE34E9164C06A85DF7C2F0433645C7875A127B820115303404809B3B6",
			"parts": {
				"total": "1",
				"hash": "9D20A7D2A32D9EA1E5D567E9AE546889581A45F316C2E10755535BD0737921C0"
			}
		},
		"block_header": {
			"version": {
				"block": "10",
				"app": "0"
			},
			"chain_id": "chain-qILMsV",
			"height": "5812561",
			"time": "2024-06-25T05:57:42.742413604Z",
			"last_block_id": {
				"hash": "E25975AF1C35E24DE22649969FB36BF7BD1E4C3E8143510FB194E74B716EED54",
				"parts": {
					"total": "1",
					"hash": "87B8CE2C2EE639DC47DC58F11FFBEFCF8B97F018D30D2A0F143E6D874DCC3696"
				}
			},
			"last_commit_hash": "10A55872C66C92C3738A51DA55D2142B15C0EFAF455F62FD5AAAC28E99F01319",
			"data_hash": "35A93BA81CF8DA26802C103AD141147820EE9975F667AA6DFA56795198768480",
			"validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
			"next_validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
			"consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
			"app_hash": "C42510F6D4B9BBA3BE08E19EF3E36A1E31701AC9A510A314EB5E2F2DCA032180",
			"last_results_hash": "352E083368129F73731739BBD04F7964DD3CA9FB4665BC703C305CEEFE5B9A69",
			"evidence_hash": "",
			"proposer_address": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7"
		}
	}, {
		"block_hash": "E25975AF1C35E24DE22649969FB36BF7BD1E4C3E8143510FB194E74B716EED54",
		"block_num": 5812560,
		"app_hash": "EAA672B0BD8B5F7A8708BCC3A9C6EB3989A69120F3B280DA62F8A89FAE1FF120",
		"proposer": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7",
		"num_txs": 1,
		"block_size": 8325,
		"block_id": {
			"hash": "E25975AF1C35E24DE22649969FB36BF7BD1E4C3E8143510FB194E74B716EED54",
			"parts": {
				"total": "1",
				"hash": "87B8CE2C2EE639DC47DC58F11FFBEFCF8B97F018D30D2A0F143E6D874DCC3696"
			}
		},
		"block_header": {
			"version": {
				"block": "10",
				"app": "0"
			},
			"chain_id": "chain-qILMsV",
			"height": "5812560",
			"time": "2024-06-25T05:57:08.278720595Z",
			"last_block_id": {
				"hash": "4E81593DB5A09DA42A9E4514476C9FBC6C5255F431E361A3B8117E6F46A479A8",
				"parts": {
					"total": "1",
					"hash": "4CCD089EF35C506527DCFCDC253E6E6AA3B8AB74F1F00360531CB1F792AB0820"
				}
			},
			"last_commit_hash": "5F013BD68C8A63C7A7C313377D6DC3FF3C9FA3744F24D94691987CE6DBE9733D",
			"data_hash": "B1D2CFD492580F5755026B390C3B13E49A83D8DAD1630C42208E875F754A74CC",
			"validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
			"next_validators_hash": "F0A2F1BB46A9C113FF1236BD837F2FFDF2D1F4EDA3BDBE06AC5378F76BD96930",
			"consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
			"app_hash": "EAA672B0BD8B5F7A8708BCC3A9C6EB3989A69120F3B280DA62F8A89FAE1FF120",
			"last_results_hash": "352E083368129F73731739BBD04F7964DD3CA9FB4665BC703C305CEEFE5B9A69",
			"evidence_hash": "",
			"proposer_address": "09EF1DB6B67D1CBF7EBA6BD9B204611848993DF7"
		}
	}]
}
```


<h3 id="3.1">3.1 统计交易量</h3>

* `GET /api/chain/statistics`
* 无参数
* Request: `/api/chain/statistics`
* Response:
```json
{
  "active_addrs": 124934,
  "total_txs": 2652844,
  "daily_txs": 798
}
```

<h3 id="3.2">3.2 地址总量</h3>

* `GET /api/chain/address/count`
* 无参数
* Request: `/api/chain/address/count`
* Response:
```json
{
    "count": 125003
}
```

<h3 id="4.1">4.1 获取资产信息</h3>

* `GET /api/assets`


| 参数        | 类型     | 必传 | 说明         |
|-----------|--------|----|------------|
| address   | string | N  | asset地址    |
| page      | number | N  | 页码，缺省值为1   |
| page_size | number | N  | 页大小，缺省值为10 |

* Request: `/api/assets?page=1&page_size=2`
* Response:
```json
{
  "total": 247,
  "page": 1,
  "page_size": 2,
  "data": [{
    "asset": "1gMObKDNRPAj_M7uMjLO94zkIyTXukR0x5o2XuCMX14=",
    "tx": "4b3002d2ac493b9a07136f3d496a803925665df83a6e8f6345050352dcef5fc7",
    "block": "A9A627B303D570AE4FA16B5044B361C6550D9152BB83B1CD78F238436355D9D7",
    "issuer": "fra1y6celqr7zz7qwz29mk0x2j9q0eq3jmd7rxynxvur7xaqdn0a6qmqtdtmj7",
    "height": 5687046,
    "timestamp": 1716454143,
    "ty": 1,
    "value": {
      "IssueAsset": {
        "body": {
          "code": {
            "val": [214, 3, 14, 108, 160, 205, 68, 240, 35, 252, 206, 238, 50, 50, 206, 247, 140, 228, 35, 36, 215, 186, 68, 116, 199, 154, 54, 94, 224, 140, 95, 94]
          },
          "num_outputs": 1,
          "records": [
            [{
              "id": null,
              "record": {
                "amount": {
                  "NonConfidential": "500000000000000"
                },
                "asset_type": {
                  "NonConfidential": [214, 3, 14, 108, 160, 205, 68, 240, 35, 252, 206, 238, 50, 50, 206, 247, 140, 228, 35, 36, 215, 186, 68, 116, 199, 154, 54, 94, 224, 140, 95, 94]
                },
                "public_key": "JrGfgH4QvAcJRd2eZUigfkEZbb4ZiTMzg_G6Bs390DY="
              }
            }, null]
          ],
          "seq_num": 139351
        },
        "pubkey": {
          "key": "JrGfgH4QvAcJRd2eZUigfkEZbb4ZiTMzg_G6Bs390DY="
        },
        "signature": "Z4mrYEvyvnk3Ove56Y5596PLWnvfTD7G-e6DYj6jx44P8dpMkjY0GAY2e_rtHs6AJkFanPSCf9BKGIIqpACBDA=="
      }
    }
  }, {
    "asset": "1gMObKDNRPAj_M7uMjLO94zkIyTXukR0x5o2XuCMX14=",
    "tx": "81d49093a741be310fcf3ff78c14faff49f585c344508fa8a0daecd3b285fee3",
    "block": "A7FE4A644E36FFFA993898487954DEE71102837E6BCA62150C6726D11D6F0366",
    "issuer": "fra1y6celqr7zz7qwz29mk0x2j9q0eq3jmd7rxynxvur7xaqdn0a6qmqtdtmj7",
    "height": 5687045,
    "timestamp": 1716454126,
    "ty": 1,
    "value": {
      "IssueAsset": {
        "body": {
          "code": {
            "val": [214, 3, 14, 108, 160, 205, 68, 240, 35, 252, 206, 238, 50, 50, 206, 247, 140, 228, 35, 36, 215, 186, 68, 116, 199, 154, 54, 94, 224, 140, 95, 94]
          },
          "num_outputs": 1,
          "records": [
            [{
              "id": null,
              "record": {
                "amount": {
                  "NonConfidential": "500000000000000"
                },
                "asset_type": {
                  "NonConfidential": [214, 3, 14, 108, 160, 205, 68, 240, 35, 252, 206, 238, 50, 50, 206, 247, 140, 228, 35, 36, 215, 186, 68, 116, 199, 154, 54, 94, 224, 140, 95, 94]
                },
                "public_key": "JrGfgH4QvAcJRd2eZUigfkEZbb4ZiTMzg_G6Bs390DY="
              }
            }, null]
          ],
          "seq_num": 139351
        },
        "pubkey": {
          "key": "JrGfgH4QvAcJRd2eZUigfkEZbb4ZiTMzg_G6Bs390DY="
        },
        "signature": "Z4mrYEvyvnk3Ove56Y5596PLWnvfTD7G-e6DYj6jx44P8dpMkjY0GAY2e_rtHs6AJkFanPSCf9BKGIIqpACBDA=="
      }
    }
  }]
}
```

<h3 id="5.1">5.1 获取FRA市场信息</h3>

* `GET /api/coins/:id/market_chart`


| 参数          | 类型     | 必传 | 说明              |
|-------------|--------|----|-----------------|
| id          | string | Y  | 币种              |
| vs_currency | string | N  | 计价货币，缺省值为"usd"  |
| interval    | string | N  | 时间间隔，缺省为"daily" |
| days        | number | N  | 天数，缺省值为7        |

* Request: `/api/coins/findora/market_chart?vs_currency=usd&days=7&interval=daily`
* Response:
```json
{
  "market_caps":[
     [
        1718841600000,
        12076960.654386723
     ],
     [
        1718928000000,
        12502620.19204148
     ],
     [
        1719014400000,
        12518619.67665847
     ],
     [
        1719100800000,
        12756431.19328877
     ],
     [
        1719187200000,
        12478551.446567249
     ],
     [
        1719273600000,
        10902644.116966683
     ],
     [
        1719360000000,
        10841063.701286633
     ],
     [
        1719415382000,
        10689314.764516797
     ]
  ],
  "prices":[
     [
        1718841600000,
        0.0010421953662283902
     ],
     [
        1718928000000,
        0.0010786349850002864
     ],
     [
        1719014400000,
        0.001075916897093825
     ],
     [
        1719100800000,
        0.0011022425089767258
     ],
     [
        1719187200000,
        0.0010769704462329777
     ],
     [
        1719273600000,
        0.0009381464216474152
     ],
     [
        1719360000000,
        0.0009336248576932378
     ],
     [
        1719415382000,
        0.0009218445536291912
     ]
  ],
  "total_volumes":[
     [
        1718841600000,
        255031.1386785886
     ],
     [
        1718928000000,
        77237.26698759367
     ],
     [
        1719014400000,
        353159.57762952574
     ],
     [
        1719100800000,
        439593.60549064353
     ],
     [
        1719187200000,
        430793.7394006403
     ],
     [
        1719273600000,
        357770.9091104639
     ],
     [
        1719360000000,
        466249.0112276624
     ],
     [
        1719415382000,
        480190.8136610133
     ]
  ]
}
```

<h3 id="5.2">5.2 获取FRA单价</h3>

* `GET /api/simple/price`


| 参数            | 类型     | 必传 | 说明               |
|---------------|--------|----|------------------|
| ids           | string | N  | 币种，缺省值为"findora" |
| vs_currencies | string | N  | 计价货币，缺省值为"usd"   |

* Request: `/api/simple/price?ids=findora&vs_currencies=usd`
* Response:
```json
{
    "findora": {
      "usd": 0.00092216
    }
}
```

# Findora Explorer V2 API Spec

* [1.1 根据地址获取Asset](#1.1)
* [1.2 获取asset集合](#1.2)
* [1.3 获取claim记录](#1.3)
* [1.4 根据交易hash获取claim记录](#1.4)
* [1.5 根据交易hash获取delegate记录](#1.5)
* [1.6 获取delegate记录](#1.6)
* [1.7 获取undelegate记录](#1.7)
* [1.8 根据交易hash获取undelegate记录](#1.8)
* [1.9 获取交易](#1.9)
* [1.10 获取用户发出的prism交易记录](#1.10)

<h3 id="1.1">1.1 根据地址查询Asset</h3>

* `GET /api/v2/asset`

* 参数

| 参数        | 类型     | 必传 | 说明       |
|-----------|--------|----|----------|
| address   | string | Y  | asset地址  |
| page      | number | N  | 页码，默认1   |
| page_size | number | N  | 页大小，默认10 |

* Request: `/api/v2/asset?address=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=&page=1&page_size=2`
* Response:
  * 按区块高度降序排列
```json
{
  "code": 200,
  "data": {
    "assets": [{
      "asset": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      "block": "58A7A6E2600B74C238207848B5455CEB31B3DCD53BA61F902EE4ABE0CD2FEF0D",
      "height": 1451661,
      "issuer": "fra1rkvlrs8j8y7rlud9qh6ndg5nr4ag7ar4640dr8h0ys6zfrwv25as42zptu",
      "timestamp": 1639902625,
      "tx": "199280b6c417a92e694ea5374b4cbb939914c127d64be0a15408dd446931bde1",
      "ty": 0,
      "value": {
        "DefineAsset": {
          "body": {
            "asset": {
              "asset_rules": {
                "decimals": 6,
                "max_units": "21000000000001000",
                "transfer_multisig_rules": null,
                "transferable": true,
                "updatable": false
              },
              "code": {
                "val": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
              },
              "issuer": {
                "key": "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs="
              },
              "memo": "FRA"
            }
          },
          "pubkey": {
            "key": "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs="
          },
          "signature": "evtO-ogJEv7iVOAJxb6YEisUEXWW2EMVv9E8C_Z9abV2jp-3hwARx2W6OwSLhoUJBrRq1wZLWBnc-Ml5YxXBCQ=="
        }
      }
    }, {
      "asset": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      "block": "58A7A6E2600B74C238207848B5455CEB31B3DCD53BA61F902EE4ABE0CD2FEF0D",
      "height": 1451661,
      "issuer": "fra1rkvlrs8j8y7rlud9qh6ndg5nr4ag7ar4640dr8h0ys6zfrwv25as42zptu",
      "timestamp": 1639902625,
      "tx": "199280b6c417a92e694ea5374b4cbb939914c127d64be0a15408dd446931bde1",
      "ty": 1,
      "value": {
        "IssueAsset": {
          "body": {
            "code": {
              "val": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            },
            "num_outputs": 2,
            "records": [
              [{
                "id": null,
                "record": {
                  "amount": {
                    "NonConfidential": "10500000000000000"
                  },
                  "asset_type": {
                    "NonConfidential": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                  },
                  "public_key": "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs="
                }
              }, null],
              [{
                "id": null,
                "record": {
                  "amount": {
                    "NonConfidential": "10500000000000000"
                  },
                  "asset_type": {
                    "NonConfidential": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                  },
                  "public_key": "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs="
                }
              }, null]
            ],
            "seq_num": 0
          },
          "pubkey": {
            "key": "HZnxwPI5PD_xpQX1NqKTHXqPdHXVXtGe7yQ0JI3MVTs="
          },
          "signature": "Z6_Dfdkx9CMiao1doiCz6tc8zH4gOCuNSc61etAzKu4xWqlRSzS5NrUtKJ16z8DLqhjm8AKpVfbncfJkQCXwCQ=="
        }
      }
    }],
    "page": 1,
    "page_size": 2,
    "total": 8
  },
  "message": ""
}
```



<h3 id="1.2">1.2 获取asset集合</h3>

* `GET /api/v2/asset/list`
* 参数

| 参数        | 类型     | 必传 | 说明       |
|-----------|--------|----|----------|
| page      | number | N  | 页码，默认1   |
| page_size | number | N  | 页大小，默认10 |

* Request: `/api/v2/asset/list?page=1&page_size=2`
* Response:
    * 按区块高度降序排列
```json
{
	"code": 200,
	"data": {
		"assets": [{
			"asset": "bVXSkVh0hFHX_oUYEfC85_gsjbK5ZL_PovE0Umy-siw=",
			"block": "4DC1972393E2046725C2787BF63160070DCC290F52988B4E6B303B32882E2246",
			"height": 5261130,
			"issuer": "fra166dndg5j09deghynastdjam8d0eaqkaqx95mukrscvsm3kecqqws3tfgms",
			"timestamp": 1707684612,
			"tx": "a666e141715a9d2d1697808ecae45e6b8274df3ee0abb341251ea4e34777cf9c",
			"ty": 1,
			"value": {
				"IssueAsset": {
					"body": {
						"code": {
							"val": [109, 85, 210, 145, 88, 116, 132, 81, 215, 254, 133, 24, 17, 240, 188, 231, 248, 44, 141, 178, 185, 100, 191, 207, 162, 241, 52, 82, 108, 190, 178, 44]
						},
						"num_outputs": 1,
						"records": [
							[{
								"id": null,
								"record": {
									"amount": {
										"NonConfidential": "50000000000000"
									},
									"asset_type": {
										"NonConfidential": [109, 85, 210, 145, 88, 116, 132, 81, 215, 254, 133, 24, 17, 240, 188, 231, 248, 44, 141, 178, 185, 100, 191, 207, 162, 241, 52, 82, 108, 190, 178, 44]
									},
									"public_key": "1ps2opJ5W5Rck-wW2Xdna_PQW6Axab5YcMMhuNs4AB0="
								}
							}, null]
						],
						"seq_num": 130486
					},
					"pubkey": {
						"key": "1ps2opJ5W5Rck-wW2Xdna_PQW6Axab5YcMMhuNs4AB0="
					},
					"signature": "RjCH51IUklW4Npv0SgV9hkl6ejg3bqNzcYCp1DHrjGX1yt_oai6BpRwTn2PZYa8t9dSHUCBwRKmLgqydlXAKAA=="
				}
			}
		}, {
			"asset": "bVXSkVh0hFHX_oUYEfC85_gsjbK5ZL_PovE0Umy-siw=",
			"block": "8D5622D4C12ECF7A370457BE156D09C54C6ABAA90F0203B5D24C7DD28B92147B",
			"height": 5261099,
			"issuer": "fra166dndg5j09deghynastdjam8d0eaqkaqx95mukrscvsm3kecqqws3tfgms",
			"timestamp": 1707684053,
			"tx": "34d40db1f8d1e3ca202ea8284a5efcaf73a6e212fc145e0d34012e22a4972bc3",
			"ty": 1,
			"value": {
				"IssueAsset": {
					"body": {
						"code": {
							"val": [109, 85, 210, 145, 88, 116, 132, 81, 215, 254, 133, 24, 17, 240, 188, 231, 248, 44, 141, 178, 185, 100, 191, 207, 162, 241, 52, 82, 108, 190, 178, 44]
						},
						"num_outputs": 1,
						"records": [
							[{
								"id": null,
								"record": {
									"amount": {
										"NonConfidential": "10000000000000"
									},
									"asset_type": {
										"NonConfidential": [109, 85, 210, 145, 88, 116, 132, 81, 215, 254, 133, 24, 17, 240, 188, 231, 248, 44, 141, 178, 185, 100, 191, 207, 162, 241, 52, 82, 108, 190, 178, 44]
									},
									"public_key": "1ps2opJ5W5Rck-wW2Xdna_PQW6Axab5YcMMhuNs4AB0="
								}
							}, null]
						],
						"seq_num": 130485
					},
					"pubkey": {
						"key": "1ps2opJ5W5Rck-wW2Xdna_PQW6Axab5YcMMhuNs4AB0="
					},
					"signature": "9TFPa9ARxGVh8LZhJEqh1ZBL7hVhD0XkNhYoOoU9z2vtjwykRcEu5I_fdrea3fmgWRMFhLpz9BLDGpK9hTT-BQ=="
				}
			}
		}],
		"page": 1,
		"page_size": 2,
		"total": 242
	},
	"message": ""
}
```

<h3 id="1.3">1.3 获取claim记录</h3>

* `GET /api/v2/staking/claims`
* 参数

| 参数        | 类型     | 必传 | 说明                   |
|-----------|--------|----|----------------------|
| address   | string | N  | 用户地址，不传则返回所有的claim记录 |
| page      | number | N  | 页码，默认1               |
| page_size | number | N  | 页大小，默认10             |

* Request: 
  * e.g. 获取claim记录：`/api/v2/staking/claims?page=1&page_size=10`
  * e.g. 获取用户的claim记录：`/api/v2/staking/claims?address=fra1zfqvdavujcjd9eumz8y0vjsr5js8hsxmv0vgldjsu0ddjdm2547qf3f6tg&page=1&page_size=10`
* Response:
  * 按区块高度降序排列
```json
{
	"code": 200,
	"data": {
		"data": [{
			"amount": 0,
			"block_hash": "23D925287A2C08036E3C8D2809CFB73FABA29C7FC95809C0DCAF1A1BB0482E11",
			"from": "fra1zfqvdavujcjd9eumz8y0vjsr5js8hsxmv0vgldjsu0ddjdm2547qf3f6tg",
			"height": 5468968,
			"timestamp": 1711852829,
			"tx_hash": "609081d6a52081b82900fffa3a11513d60dc751dc28a9cf01857b9ffb6bc08f0",
			"value": {
				"Claim": {
					"body": {
						"amount": null,
						"nonce": [
							[51, 68, 235, 174, 85, 98, 153, 128], 133140
						]
					},
					"pubkey": "EkDG9ZyWJNLnmxHI9koDpKB7wNtj2I-2UOPa2TdqpXw=",
					"signature": "WXVTtSfckHTm_M8POcE3lfBAkS32pYFPlzrBau_Vr8YOWYS440F7SSnZD9DAmfwCrlbd2py3qTUC60B55TmWCw==",
					"td_addr": [253, 140, 101, 99, 74, 157, 136, 153, 250, 20, 32, 1, 119, 175, 25, 210, 79, 110, 28, 55]
				}
			}
		}, {
			"amount": 0,
			"block_hash": "F6CDAA225C47974B7864075A793C5D17D0E2680C8AF4237A2461C4665FF16F1D",
			"from": "fra1zfqvdavujcjd9eumz8y0vjsr5js8hsxmv0vgldjsu0ddjdm2547qf3f6tg",
			"height": 5450196,
			"timestamp": 1711475308,
			"tx_hash": "4dd36fda15ea9162493a5e5c3eaeadd9fd3b663a4495be09c831317e19a84d64",
			"value": {
				"Claim": {
					"body": {
						"amount": null,
						"nonce": [
							[61, 227, 178, 41, 152, 152, 229, 16], 132890
						]
					},
					"pubkey": "EkDG9ZyWJNLnmxHI9koDpKB7wNtj2I-2UOPa2TdqpXw=",
					"signature": "bkvcDCP-9I95xMaciC78Kum60xIdcFdfSdVG3jDT8XqDflmmtp-4Ssjma5bzt8UNDnn2PI6Yw0_er4JuFHCuBA==",
					"td_addr": [0, 14, 51, 171, 116, 113, 24, 111, 59, 29, 233, 252, 8, 187, 156, 72, 15, 69, 53, 144]
				}
			}
		}],
		"page": 1,
		"page_size": 2,
		"total": 474
	},
	"message": ""
}
```

<h3 id="1.4">1.4 根据交易hash获取claim记录</h3>

* `GET /api/v2/staking/claim/:tx_hash`
* 参数

| 参数      | 类型     | 必传 | 说明   |
|---------|--------|----|------|
| tx_hash | string | Y  | 交易哈希 |


* Request: `/api/v2/staking/claim/4dd36fda15ea9162493a5e5c3eaeadd9fd3b663a4495be09c831317e19a84d64`
* Response:
```json
{
	"code": 200,
	"data": {
		"amount": 0,
		"block_hash": "F6CDAA225C47974B7864075A793C5D17D0E2680C8AF4237A2461C4665FF16F1D",
		"from": "fra1zfqvdavujcjd9eumz8y0vjsr5js8hsxmv0vgldjsu0ddjdm2547qf3f6tg",
		"height": 5450196,
		"timestamp": 1711475308,
		"tx_hash": "4dd36fda15ea9162493a5e5c3eaeadd9fd3b663a4495be09c831317e19a84d64",
		"value": {
			"Claim": {
				"body": {
					"amount": null,
					"nonce": [
						[61, 227, 178, 41, 152, 152, 229, 16], 132890
					]
				},
				"pubkey": "EkDG9ZyWJNLnmxHI9koDpKB7wNtj2I-2UOPa2TdqpXw=",
				"signature": "bkvcDCP-9I95xMaciC78Kum60xIdcFdfSdVG3jDT8XqDflmmtp-4Ssjma5bzt8UNDnn2PI6Yw0_er4JuFHCuBA==",
				"td_addr": [0, 14, 51, 171, 116, 113, 24, 111, 59, 29, 233, 252, 8, 187, 156, 72, 15, 69, 53, 144]
			}
		}
	},
	"message": ""
}
```

<h3 id="1.5">1.5 根据交易hash获取delegate记录</h3>

* `GET /api/v2/staking/delegation/:tx_hash`
* 参数

| 参数      | 类型     | 必传 | 说明   |
|---------|--------|----|------|
| tx_hash | string | Y  | 交易哈希 |

* Request: `/api/v2/staking/delegation/e2cd3ec8c29d1ad45307739785b8dc2e776e8032393c57b828d43d7db8fe3b62`
* Response:
```json
{
	"code": 200,
	"data": {
		"amount": 408000000,
		"block_hash": "1BCCEE75342A238AC3836668D83A2A48E158A570B5644C4E046E07DF97B89A93",
		"from": "fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t",
		"height": 5467159,
		"new_validator": "",
		"timestamp": 1711816770,
		"tx_hash": "e2cd3ec8c29d1ad45307739785b8dc2e776e8032393c57b828d43d7db8fe3b62",
		"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590",
		"value": {
			"Delegation": {
				"body": {
					"amount": 408000000,
					"new_validator": null,
					"nonce": [
						[224, 41, 94, 168, 98, 48, 85, 14], 133117
					],
					"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590"
				},
				"pubkey": "YNk8Fsmc_vUO_UsbOpIQ_-oxPT9OPRNaUXFX-CEEicc=",
				"signature": "wpofVSep2i1ygaxpMDx4Zbl0Um6odO8g1hSpdkjfbA7E3y6UwPth0m-wu03sS_JM0hdrnL4cik5hVvPbWFdRBg==",
				"v_signature": null
			}
		}
	},
	"message": ""
}
```

<h3 id="1.6">1.6 获取delegate记录</h3>

* `GET /api/v2/staking/delegations`
* 参数

| 参数        | 类型     | 必传 | 说明                   |
|-----------|--------|----|----------------------|
| address   | string | Y  | 用户地址，不传则返回所有的claim记录 |
| page      | number | N  | 页码，默认1               |
| page_size | number | N  | 页大小，默认10             |

* Request:
  * 获取delegate记录： `/api/v2/staking/delegations?page=1&page_size=2`
  * 获取用户的delegate记录： `/api/v2/staking/delegations?address=fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t&page=1&page_size=2`
* Response:
```json
{
	"code": 200,
	"data": {
		"items": [{
			"amount": 408000000,
			"block_hash": "1BCCEE75342A238AC3836668D83A2A48E158A570B5644C4E046E07DF97B89A93",
			"from": "fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t",
			"height": 5467159,
			"new_validator": "",
			"timestamp": 1711816770,
			"tx_hash": "e2cd3ec8c29d1ad45307739785b8dc2e776e8032393c57b828d43d7db8fe3b62",
			"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590",
			"value": {
				"Delegation": {
					"body": {
						"amount": 408000000,
						"new_validator": null,
						"nonce": [
							[224, 41, 94, 168, 98, 48, 85, 14], 133117
						],
						"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590"
					},
					"pubkey": "YNk8Fsmc_vUO_UsbOpIQ_-oxPT9OPRNaUXFX-CEEicc=",
					"signature": "wpofVSep2i1ygaxpMDx4Zbl0Um6odO8g1hSpdkjfbA7E3y6UwPth0m-wu03sS_JM0hdrnL4cik5hVvPbWFdRBg==",
					"v_signature": null
				}
			}
		}, {
			"amount": 2010000000,
			"block_hash": "AA2114E42F636679B8C67476725010ED846A690CE31C241F7830C6E2CD85141C",
			"from": "fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t",
			"height": 5464300,
			"new_validator": "",
			"timestamp": 1711758769,
			"tx_hash": "08933e65ecf6a7c4c7f1b617f4823177a28e5b6c0ab028f1d889e8af0012e8e5",
			"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590",
			"value": {
				"Delegation": {
					"body": {
						"amount": 2010000000,
						"new_validator": null,
						"nonce": [
							[6, 244, 169, 33, 144, 151, 53, 186], 133073
						],
						"validator": "000e33ab7471186f3b1de9fc08bb9c480f453590"
					},
					"pubkey": "YNk8Fsmc_vUO_UsbOpIQ_-oxPT9OPRNaUXFX-CEEicc=",
					"signature": "swLH39i-eeWhx5KhccNNHA71443WBwni1pG-u1UP3Olme8jaxjrcI1eca6krH14fuSP1x3iBhhsayBDwBZyPDw==",
					"v_signature": null
				}
			}
		}],
		"page": 1,
		"page_size": 2,
		"total": 272
	},
	"message": ""
}
```
<h3 id="1.7">1.7 获取undelegate记录</h3>

* `GET /api/v2/staking/undelegations`
* 参数

| 参数        | 类型     | 必传 | 说明                   |
|-----------|--------|----|----------------------|
| address   | string | Y  | 用户地址，不传则返回所有的claim记录 |
| page      | number | N  | 页码，默认1               |
| page_size | number | N  | 页大小，默认10             |

* Request:
  * 获取undelegate记录： `/api/v2/staking/undelegations?page=1&page_size=2`
  * 获取用户的undelegate记录：`/api/v2/staking/undelegations?address=fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t&page=1&page_size=2`
* Response:
  * 按区块高度降序排列
```json
{
	"code": 200,
	"data": {
		"items": [{
			"amount": 949417158152,
			"block_hash": "B93EA7CF643F6A26904F8A9429E12C013E90980EF6C24D0158E5D945FF2C3F8D",
			"from": "fra1mmcy767sy0shh72jla0pluhfhfydk5w73gq0m3j2j06kd4ssergqrpa57g",
			"height": 5471746,
			"new_delegator": "0vGE7s9Z8OpPoS6RiAbLN3qZUGfBSL6PRxXMKT-lhMA=",
			"target_validator": "7C77CF71CF6CBD04885E32FC49EDA367F7BC3C65",
			"timestamp": 1711908142,
			"tx_hash": "e223ce6af861769a440f0e5599fce7ef4d09d1048f5f51fd4347ca909de87d95",
			"value": {
				"UnDelegation": {
					"body": {
						"nonce": [
							[38, 244, 218, 29, 71, 253, 67, 151], 133195
						],
						"pu": {
							"am": 949417158152,
							"new_delegator_id": "0vGE7s9Z8OpPoS6RiAbLN3qZUGfBSL6PRxXMKT-lhMA=",
							"target_validator": [124, 119, 207, 113, 207, 108, 189, 4, 136, 94, 50, 252, 73, 237, 163, 103, 247, 188, 60, 101]
						}
					},
					"pubkey": "3vBPa9Aj4Xv5Uv9eH_LpukjbUd6KAP3GSpP1ZtYQyNA=",
					"signature": "x3GM0eOp_FljztQ3wXNk7DD50p9-3cLFUXBUqRAu-gGaf9ZUBaG547yhHb-sUdbUZXaFjCcb9TsT0m2g5ZR1Dw=="
				}
			}
		}, {
			"amount": 3744640540,
			"block_hash": "71E7FC9770810F482850F1487F538C129327B1A02F44EF35714046A19E9630C4",
			"from": "fra1zfqvdavujcjd9eumz8y0vjsr5js8hsxmv0vgldjsu0ddjdm2547qf3f6tg",
			"height": 5468906,
			"new_delegator": "RD8SLnaxZAs-UnLwH7_zDx1k5QRgpzJymMbslzN28PA=",
			"target_validator": "000E33AB7471186F3B1DE9FC08BB9C480F453590",
			"timestamp": 1711851641,
			"tx_hash": "62e6263a7016b1b9f4d5fc7f251bdf996aeb5fb5f9f866c48658a4c8439b61da",
			"value": {
				"UnDelegation": {
					"body": {
						"nonce": [
							[148, 137, 12, 225, 62, 224, 133, 33], 133138
						],
						"pu": {
							"am": 3744640540,
							"new_delegator_id": "RD8SLnaxZAs-UnLwH7_zDx1k5QRgpzJymMbslzN28PA=",
							"target_validator": [0, 14, 51, 171, 116, 113, 24, 111, 59, 29, 233, 252, 8, 187, 156, 72, 15, 69, 53, 144]
						}
					},
					"pubkey": "EkDG9ZyWJNLnmxHI9koDpKB7wNtj2I-2UOPa2TdqpXw=",
					"signature": "Q6WBodsrwo1BkYOi0sTS-qTRYUZo9APKG4qGzv1-5OyZ-9sVKotKi5I9jtF9wvnG_mgqN2HryCkKG57slo6PCA=="
				}
			}
		}],
		"page": 1,
		"page_size": 2,
		"total": 6182
	},
	"message": ""
}
```

<h3 id="1.8">1.8 根据交易hash获取undelegate记录</h3>

* `GET /api/v2/staking/undelegation/:tx_hash`
* 参数

| 参数      | 类型     | 必传 | 说明   |
|---------|--------|----|------|
| tx_hash | string | Y  | 交易哈希 |


* Request: `/api/v2/staking/undelegation/583d9071df8299978e227d4b62d122225e4d2bc7fce0f581b4396b5c35f63dbd`
* Response:
```json
{
	"code": 200,
	"data": {
		"amount": 96442598104,
		"block_hash": "53E86050C1B6519B8A6B6A09086810A92B97DC2905F6C14BF319B5D694CF74EF",
		"from": "fra1vrvnc9kfnnl02rhafvdn4yssll4rz0flfc73xkj3w9tlsggy38rskgv35t",
		"height": 5162087,
		"new_delegator": "5ax5Bb8xQ4Ag7eV_cA8PGAAJ2_M-BPZhrF3R5uzd17o=",
		"target_validator": "E012AA66C83999E3862C8AA534B9CE66FC14A37A",
		"timestamp": 1705780393,
		"tx_hash": "583d9071df8299978e227d4b62d122225e4d2bc7fce0f581b4396b5c35f63dbd",
		"value": {
			"UnDelegation": {
				"body": {
					"nonce": [
						[90, 0, 11, 166, 133, 201, 223, 219], 129078
					],
					"pu": {
						"am": 96442598104,
						"new_delegator_id": "5ax5Bb8xQ4Ag7eV_cA8PGAAJ2_M-BPZhrF3R5uzd17o=",
						"target_validator": [224, 18, 170, 102, 200, 57, 153, 227, 134, 44, 138, 165, 52, 185, 206, 102, 252, 20, 163, 122]
					}
				},
				"pubkey": "YNk8Fsmc_vUO_UsbOpIQ_-oxPT9OPRNaUXFX-CEEicc=",
				"signature": "0bwIKjNsCqfDMPp6iAtOM81nMHw4voGwyl_YjNazfx2A6Wt-BWWR0B-bruWANPMNncGRbBuYs2Brjigd566LAw=="
			}
		}
	},
	"message": ""
}
```

<h3 id="1.9">1.9 获取交易</h3>

* `GET /api/v2/txs`

* 参数

| 参数           | 类型     | 必传 | 说明    |
|--------------|--------|----|-------|
| block_hash   | string | N  | 区块哈希  |
| block_height | number | N  | 区块高度  |
| from         | string | N  | 发送者地址 |
| to           | string | N  | 接收者地址 |
| ty           | number | N  | 交易类型  |
| start_time   | number | N  | 开始时间戳 |
| end_time     | number | N  | 截止时间戳 |
| page         | number | N  | 页码    |
| page_size    | number | N  | 页大小   |

* Request: `/api/v2/txs?page=1&page_size=2`
* Response:
  * 按timestamp降序排列
```json
{
	"code": 200,
	"data": {
		"page": 1,
		"page_size": 2,
		"total": 2460630,
		"txs": [{
			"block_hash": "B92B8B1AB4BB145AA66E4EA245706A8F833AD7FBED02D2603CA9003CD0324E9D",
			"code": 0,
			"evm_tx_hash": "",
			"height": 5478292,
			"log": "",
			"origin": "ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4OTBiOTAiLCJnYXNfcHJpY2UiOiIweDI1NDBiZTQwMCIsImdhc19saW1pdCI6IjB4MjQ5ZjAiLCJhY3Rpb24iOnsiQ2FsbCI6IjB4YjUyZWYyOTI2NTFmODY5MjA1M2FjM2QzOTkyMWE0M2ZlMTM1MzNjOCJ9LCJ2YWx1ZSI6IjB4MCIsImlucHV0IjpbMTk4LDY0LDExNyw0NSwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwxNzUsMjUsMTA4LDQ1LDEyMCwxMTIsMTgxLDIwLDExMSwyLDIxMiwxNTUsMSwyOSwyNDksMjUsMTI0LDksNjMsMjU0LDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDE2LDBdLCJzaWduYXR1cmUiOnsidiI6NDM0MCwiciI6IjB4OWUwNmExYjhjYzBmMjFiYzBmMTZiZTk1YzAzNjY5ZDQxMzE1NWE1M2E0ODc4MTJkNTUzNGFlY2JmNDViMjJjYiIsInMiOiIweDJiYmZkNDZjYzQwZDc5OWI2MjhlMjVmMzE4NmNjODZmNmU2YWRiZDE2NTU2ZWFmNDIwMzNmNjA4NTBiOWU4ODgifX19fX0=",
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
						"value": "MHg3MzgxYWEyMTBiMWJjMmU4NmYxMTExN2Q5ZTFlODZjYmYyYmU2ZWE2YjdhZWQwODhiZjk2N2MzNmJjZjA5MGIw"
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
			"timestamp": 1712037200,
			"tx_hash": "11969300f45e0db1b5a74a1c102a252a2b443cb5cb1346e41d30bed234b4efe3",
			"ty": 1,
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
							"input": [198, 64, 117, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 175, 25, 108, 45, 120, 112, 181, 20, 111, 2, 212, 155, 1, 29, 249, 25, 124, 9, 63, 254, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 0],
							"nonce": "0x90b90",
							"signature": {
								"r": "0x9e06a1b8cc0f21bc0f16be95c03669d413155a53a487812d5534aecbf45b22cb",
								"s": "0x2bbfd46cc40d799b628e25f3186cc86f6e6adbd16556eaf42033f60850b9e888",
								"v": 4340
							},
							"value": "0x0"
						}
					}
				}
			}
		}, {
			"block_hash": "8C4C163BCC93062974D2D5E6165DD0C399EAA1BAF615901610197C9F37509E2E",
			"code": 0,
			"evm_tx_hash": "",
			"height": 5478291,
			"log": "",
			"origin": "ZXZtOnsic2lnbmF0dXJlIjpudWxsLCJmdW5jdGlvbiI6eyJFdGhlcmV1bSI6eyJUcmFuc2FjdCI6eyJub25jZSI6IjB4OTBiOGYiLCJnYXNfcHJpY2UiOiIweDI1NDBiZTQwMCIsImdhc19saW1pdCI6IjB4MjQ5ZjAiLCJhY3Rpb24iOnsiQ2FsbCI6IjB4YjUyZWYyOTI2NTFmODY5MjA1M2FjM2QzOTkyMWE0M2ZlMTM1MzNjOCJ9LCJ2YWx1ZSI6IjB4MCIsImlucHV0IjpbMTk4LDY0LDExNyw0NSwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwxNiwwLDAsMCwwLDAsMCwwLDAsMCwwLDAsMCwwLDYsNTAsMTg2LDE2Miw5OCwxNTMsMjAxLDE1MSw0NiwyMTIsMjE3LDE3NSwyNTAsNjMsMjA4LDg3LDE2NywzNCw4MiwyNTVdLCJzaWduYXR1cmUiOnsidiI6NDM0MCwiciI6IjB4OTc0ZDBkYjQ1ZTQxMDM2ZmI3YjFmM2Q3ODNkZGE1ZTVlYjM4NTM4YzlmMzNhYzdmNjkxODk2YmY3OGU0MjhlNCIsInMiOiIweDFlOWYxMWVhYjRmMDU2MGFhNDU3ZTQ5OGI0ODg5ZmI0N2JhODRiNjY3MjhkODJkNjMwNjNhMWZjZjM0NzI3MjYifX19fX0=",
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
						"value": "MHgzNGU4NzRjNzAxNDdlZmIxOGE5ODM0ZDkxMTg1YTI3ZTI1YjFjZWY1YTAwOTBjYTBmNzVjMjMyYTAxM2QyMWFi"
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
			"timestamp": 1712037185,
			"tx_hash": "039a2481e3b7b81dfb344e0627fca9bccb8e249f18b5892fd234833e128995a5",
			"ty": 1,
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
							"nonce": "0x90b8f",
							"signature": {
								"r": "0x974d0db45e41036fb7b1f3d783dda5e5eb38538c9f33ac7f691896bf78e428e4",
								"s": "0x1e9f11eab4f0560aa457e498b4889fb47ba84b66728d82d63063a1fcf3472726",
								"v": 4340
							},
							"value": "0x0"
						}
					}
				}
			}
		}]
	},
	"message": ""
}
```

<h3 id="1.10">1.10 获取用户发出的prism交易记录</h3>

* `GET /v2/tx/prism/send`



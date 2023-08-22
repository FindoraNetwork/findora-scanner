# Findora Scanner V2 API Spec

## Transaction
### [2.3.Staking](#2.3)
* [2.3.1 Get delegation tx by hash](#2.3.1)
### [2.4.UnStaking](#2.4)
* [2.4.1 Get undelegation tx by hash](#2.4.1)
### [2.5.Rewards](#2.5)
* [2.5.1 Get claim tx by hash](#2.5.1)
### [2.6.Native To EVM](#2.6)
* [2.6.1 Get native to evm tx by hash](#2.6.1)
### [2.7.Asset](#2.7)
* [2.7.1 Get assets](#2.7.1)

<h2 id="2.3">2.3 Staking</h2>
<h3 id="2.3.1">2.3.1 Get delegation tx by hash</h3>

* `GET /api/v2/tx/delegation/:hash`

| 参数   | 类型     | 说明               | 必传 |
|------|--------|------------------|----|
| hash | string | transaction hash | Y  |

* Request:
  * `http://localhost/api/v2/tx/delegation/5ca21da32f029318e811db17f9240659c98e86169bd42c1b7901afff71b425b3`
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

<h2 id="2.4">2.4 Unstaking</h2>
<h3 id="2.4.1">2.4.1 Get undelegation tx by hash</h3>

* `GET /api/v2/tx/undelegation/:hash`

| 参数   | 类型     | 说明               | 必传 |
|------|--------|------------------|----|
| hash | string | transaction hash | Y  |

* Request:
  * `http://localhost/api/v2/tx/undelegation/91001c320aa13cef240f00b1cd941a429b72de9f089ccd301111998aa55d6562`
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

<h2 id="2.5">2.5 Rewards</h2>
<h3 id="2.5.1">2.5.1 Get claim tx by hash</h3>

* `GET /api/v2/tx/claim/:hash`

| 参数   | 类型     | 说明               | 必传 |
|------|--------|------------------|----|
| hash | string | transaction hash | Y  |

* Request:
  * `http://localhost/api/v2/tx/claim/114710d4828472bc3e2da38089b70efe92d47f25b8b0439b918de227a9cbf392`
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
<h2 id="2.6">2.6 Native To EVM</h2>
<h3 id="2.6.1">2.6.1 Get native to evm tx by hash</h3>

* `GET /api/v2/tx/n2e/:hash`

| 参数   | 类型     | 说明               | 必传 |
|------|--------|------------------|----|
| hash | string | transaction hash | Y  |

* Request:
  * `http://localhost/api/v2/tx/n2e/e5fb78efd8dac071a7d8e5e9217971fe552302f74aad811106a314f1c3029ccd`
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

<h2 id="2.7">2.7 Asset</h2>
<h3 id="2.7.1">2.7.1 Get assets</h3>

* `/v2/asset`

| 参数        | 类型     | 说明            | 必传 |
|-----------|--------|---------------|----|
| address   | string | asset address | Y  |
| page      | number | page index    | N  |
| page_size | string | page size     | N  |

* Request:
  * `http://localhost/api/v2/asset?address=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=&page=1&page_size=10`
* Response:
```json
{
	"code": 200,
	"data": {
		"assets": [{
			"asset": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
			"block": "DDB245E3E049987A462C493C37B13D33126B92FEE2163F3387890E672A965472",
			"height": 1,
			"issuer": "fra1xklsu8an2y4yd2e8kc43xspx54vrppp4l8fhtpey4n9z0kvmw2tqf76l2c",
			"timestamp": 1617076716,
			"tx": "c063f65ff3b97baa2a8b7e775a4f5aa7f161a12f705205df037b8bf44e28716e",
			"ty": 0,
			"value": {
				"DefineAsset": {
					"body": {
						"asset": {
							"asset_rules": {
								"decimals": 6,
								"max_units": "21420000000000000",
								"transfer_multisig_rules": null,
								"transferable": true,
								"updatable": false
							},
							"code": {
								"val": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
							},
							"issuer": {
								"key": "Nb8OH7NRKkarJ7YrE0AmpVgwhDX503WHJKzKJ9mbcpY="
							},
							"memo": "Findora_Native_Token"
						}
					},
					"pubkey": {
						"key": "Nb8OH7NRKkarJ7YrE0AmpVgwhDX503WHJKzKJ9mbcpY="
					},
					"signature": "o0nwbFYXAsonLwjka9qjREqi2oOKxlx_hyx9g6GftqH_Nwq6GmJQc3x0-rfxVatXWgRr5oJVGPK_tFU3pycmDg=="
				}
			}
		}, {
			"asset": "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
			"block": "DDB245E3E049987A462C493C37B13D33126B92FEE2163F3387890E672A965472",
			"height": 1,
			"issuer": "fra1xklsu8an2y4yd2e8kc43xspx54vrppp4l8fhtpey4n9z0kvmw2tqf76l2c",
			"timestamp": 1617076716,
			"tx": "c063f65ff3b97baa2a8b7e775a4f5aa7f161a12f705205df037b8bf44e28716e",
			"ty": 1,
			"value": {
				"IssueAsset": {
					"body": {
						"code": {
							"val": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
						},
						"num_outputs": 1,
						"records": [
							[{
								"id": null,
								"record": {
									"amount": {
										"NonConfidential": "21000000000000000"
									},
									"asset_type": {
										"NonConfidential": [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
									},
									"public_key": "Nb8OH7NRKkarJ7YrE0AmpVgwhDX503WHJKzKJ9mbcpY="
								}
							}, null]
						],
						"seq_num": 0
					},
					"pubkey": {
						"key": "Nb8OH7NRKkarJ7YrE0AmpVgwhDX503WHJKzKJ9mbcpY="
					},
					"signature": "GLLAD9HH23bRw6YmzplogMTU6DEy_8yD27RyiyLnPF18BaGQoLvggl9w3fi2t4A3JaQT0RRxT1d8EyBbEJUsAg=="
				}
			}
		}],
		"page": 1,
		"page_size": 10,
		"total": 2
	},
	"message": ""
}
```

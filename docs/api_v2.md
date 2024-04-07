# Findora Explorer V2 API Spec

## Block
* [1.1 根据区块号获取区块(Get block by number)](#1.1)
* [1.2 根据区块哈希获取区块(Get block by hash)](#1.2)
* [1.3 获取区块(Get blocks)](#1.3)

## Transaction
* [2.1 根据交易哈希获取交易(Get transaction by hash)](#2.1)

## Staking

<h3 id="1.1">1.1 根据区块号获取区块</h3>

* `GET /api/v2/number/block`

* 参数

| 参数(param) | 类型(type) | 必传(required) | 说明  |
|-----------|----------|--------------|-----|
| num       | number   | Y            | 区块号 |

* Request: `http://localhost/api/v2/number/block?num=100`
* Response:
```json
{
  "block_hash": "E8A4A1F0A6AE1EBAC0D8CA84106985DEFA47240A2AD4E045717CD304B8EDD985",
  "block_num": 100,
  "app_hash": "759AB503F009046D19FC4A1F7B3DE5E1087FE3B59FC6D842FDA5C74264F2ABEE",
  "proposer": "FD8C65634A9D8899FA14200177AF19D24F6E1C37",
  "num_txs": 0,
  "block_size": 2305,
  "block_id": {
    "hash": "E8A4A1F0A6AE1EBAC0D8CA84106985DEFA47240A2AD4E045717CD304B8EDD985",
    "parts": {
      "total": "1",
      "hash": "4CAD8FEEDF00A02839D94AA2DE53C458C125E88F72A82F5AB414047B2B8864FC"
    }
  },
  "block_header": {
    "version": {
      "block": "10",
      "app": "0"
    },
    "chain_id": "chain-qILMsV",
    "height": "100",
    "time": "2021-03-30T05:37:46.111298958Z",
    "last_block_id": {
      "hash": "D823B5C8D961507F6DE316BA0828D494E3EEA603CEA98DAE6AC22C1FDE6C96C8",
      "parts": {
        "total": "1",
        "hash": "61D98ECCDDE383FD33513BB437D83D8F3E3638680AF13ACCD16A02F4B5EC9313"
      }
    },
    "last_commit_hash": "6E9CD69991A58B52E02BA843692F3AA43A9283C453C672AD8E99F750622562FD",
    "data_hash": "",
    "validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
    "next_validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
    "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
    "app_hash": "759AB503F009046D19FC4A1F7B3DE5E1087FE3B59FC6D842FDA5C74264F2ABEE",
    "last_results_hash": "",
    "evidence_hash": "",
    "proposer_address": "FD8C65634A9D8899FA14200177AF19D24F6E1C37"
  }
}
```

<h3 id="1.2">1.2 根据区块哈希获取区块</h3>

* `GET /api/v2/hash/block`

* 参数

| 参数(param) | 类型(type) | 必传(required) | 说明   |
|-----------|----------|--------------|------|
| hash      | string   | Y            | 区块哈希 |

* Request: `http://localhost/api/v2/hash/block?hash=E8A4A1F0A6AE1EBAC0D8CA84106985DEFA47240A2AD4E045717CD304B8EDD985`
* Response:
```json
{
  "block_hash": "E8A4A1F0A6AE1EBAC0D8CA84106985DEFA47240A2AD4E045717CD304B8EDD985",
  "block_num": 100,
  "app_hash": "759AB503F009046D19FC4A1F7B3DE5E1087FE3B59FC6D842FDA5C74264F2ABEE",
  "proposer": "FD8C65634A9D8899FA14200177AF19D24F6E1C37",
  "num_txs": 0,
  "block_size": 2305,
  "block_id": {
    "hash": "E8A4A1F0A6AE1EBAC0D8CA84106985DEFA47240A2AD4E045717CD304B8EDD985",
    "parts": {
      "total": "1",
      "hash": "4CAD8FEEDF00A02839D94AA2DE53C458C125E88F72A82F5AB414047B2B8864FC"
    }
  },
  "block_header": {
    "version": {
      "block": "10",
      "app": "0"
    },
    "chain_id": "chain-qILMsV",
    "height": "100",
    "time": "2021-03-30T05:37:46.111298958Z",
    "last_block_id": {
      "hash": "D823B5C8D961507F6DE316BA0828D494E3EEA603CEA98DAE6AC22C1FDE6C96C8",
      "parts": {
        "total": "1",
        "hash": "61D98ECCDDE383FD33513BB437D83D8F3E3638680AF13ACCD16A02F4B5EC9313"
      }
    },
    "last_commit_hash": "6E9CD69991A58B52E02BA843692F3AA43A9283C453C672AD8E99F750622562FD",
    "data_hash": "",
    "validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
    "next_validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
    "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
    "app_hash": "759AB503F009046D19FC4A1F7B3DE5E1087FE3B59FC6D842FDA5C74264F2ABEE",
    "last_results_hash": "",
    "evidence_hash": "",
    "proposer_address": "FD8C65634A9D8899FA14200177AF19D24F6E1C37"
  }
}
```

<h3 id="1.3">1.3 获取区块</h3>

* `GET /api/v2/blocks`

* 参数

| 参数(param) | 类型(type) | 必传(required) | 说明   |
|-----------|----------|--------------|------|
| page      | number   | N            | 默认1  |
| page_size | number   | N            | 默认10 |


* Request: `http://localhost/api/v2/blocks?page=1&page_size=3`
* Response:
  * 按区块号降序排列(order by block num desc)
```json
{
  "total": 250689,
  "page": 1,
  "page_size": 3,
  "data": [{
    "block_hash": "2621F0BC18118219C345669FD254993C0AA50DBBCAF735E800D85F41077E9A53",
    "block_num": 250689,
    "app_hash": "7973F6BFDEDD343D5D62487D52C8D455C813EF4C95D4087B42FFE3C6DE398E89",
    "proposer": "9ED0D8D661C99A58F78F80816968E61AAE8DC649",
    "num_txs": 1,
    "block_size": 27329,
    "block_id": {
      "hash": "2621F0BC18118219C345669FD254993C0AA50DBBCAF735E800D85F41077E9A53",
      "parts": {
        "total": "1",
        "hash": "02B417177063ABE02573919038033A0CE8C9412959F4B2A6A8A579CFF22F3E04"
      }
    },
    "block_header": {
      "version": {
        "block": "10",
        "app": "0"
      },
      "chain_id": "chain-qILMsV",
      "height": "250689",
      "time": "2023-08-16T00:03:29.377015421Z",
      "last_block_id": {
        "hash": "040BBDC5494443DCE1D018C6AB7F23C6EABAB0DFC0CF4292D094306C4800044D",
        "parts": {
          "total": "1",
          "hash": "8F22B438D22E4A45C38AE31736A1BFD33A092BD653A23081C4B69EC23FEEDD57"
        }
      },
      "last_commit_hash": "A8A28345ED16CD478AF22715E3CA577DA15E0E1DAE4155DD0478165A4723822A",
      "data_hash": "339281320D9F759E2FF6B95257A912B0D7343D76E413D66A66BDCFA642536479",
      "validators_hash": "4507ECDC1FCB92A824F0DC3082D0A528379051D78F682DE0ECD3BAE185654FE2",
      "next_validators_hash": "4507ECDC1FCB92A824F0DC3082D0A528379051D78F682DE0ECD3BAE185654FE2",
      "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
      "app_hash": "7973F6BFDEDD343D5D62487D52C8D455C813EF4C95D4087B42FFE3C6DE398E89",
      "last_results_hash": "",
      "evidence_hash": "",
      "proposer_address": "9ED0D8D661C99A58F78F80816968E61AAE8DC649"
    }
  }, 
    {
    "block_hash": "81660C94CCD8C5F6E7D764DC6BDFFB86600A14BB07A97674CDAA9ECE7C763D62",
    "block_num": 250689,
    "app_hash": "CA3D39B3D00A6C869FB4B6D8728752711C306069FC2200D88A863AFC38D926DB",
    "proposer": "8CB713C8EA32223FCAC66B966FCFA9BAEE257946",
    "num_txs": 0,
    "block_size": 2506,
    "block_id": {
      "hash": "81660C94CCD8C5F6E7D764DC6BDFFB86600A14BB07A97674CDAA9ECE7C763D62",
      "parts": {
        "total": "1",
        "hash": "187164D9E4A8354D44C84D12E076EE6F5A49C442638260E938C72FD020784BBC"
      }
    },
    "block_header": {
      "version": {
        "block": "10",
        "app": "0"
      },
      "chain_id": "chain-qILMsV",
      "height": "250689",
      "time": "2021-05-14T15:19:39.860658429Z",
      "last_block_id": {
        "hash": "D0CEE991CC9546D80AD916D642C33356EE86E19598CB68F9CAE32ABA2C16F354",
        "parts": {
          "total": "1",
          "hash": "4F656F2E9FA2200E888C597A72C1D5537FE943B4E7EA17172C4851FB12EF9335"
        }
      },
      "last_commit_hash": "8DD8A37D6ACED561F10BAC7D17232F73992F86F528AC6FE2554F734FE244479E",
      "data_hash": "",
      "validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
      "next_validators_hash": "EC4E07BBE9EE84C39699C844D32CA8739FD8145D28F6B4DE7E8DF09326212D76",
      "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
      "app_hash": "CA3D39B3D00A6C869FB4B6D8728752711C306069FC2200D88A863AFC38D926DB",
      "last_results_hash": "",
      "evidence_hash": "",
      "proposer_address": "8CB713C8EA32223FCAC66B966FCFA9BAEE257946"
    }
  }, {
    "block_hash": "A56EF951D6886B61B459564FAA7535AC307A056837E30B10F85415DEF506CB79",
    "block_num": 245136,
    "app_hash": "679BCBCFCB71F63E92BBCFE558C51BDD5BC4374163C2D228DFEAB3900B1F2030",
    "proposer": "510082967DFA7DEBA11267B26A6318D07A457B48",
    "num_txs": 1,
    "block_size": 3060,
    "block_id": {
      "hash": "A56EF951D6886B61B459564FAA7535AC307A056837E30B10F85415DEF506CB79",
      "parts": {
        "total": "1",
        "hash": "99F8154239ADAD37789D56611BC964C753AA23FD2FDDE94DE22B5521EBC1FBBA"
      }
    },
    "block_header": {
      "version": {
        "block": "10",
        "app": "0"
      },
      "chain_id": "chain-qILMsV",
      "height": "245136",
      "time": "2023-08-15T07:32:23.666042021Z",
      "last_block_id": {
        "hash": "803E22897FCC4A8147C6E2F3BC21DA3383FD5FD7CDD2B8AA4377300428F530FC",
        "parts": {
          "total": "1",
          "hash": "D8C9EC1C2C661AC8C256498C29875FBE99055A29E6684907A9FB03B6757F49EC"
        }
      },
      "last_commit_hash": "7DBAA7155E53AA9C36DC7B7C44763BCA3397BB5E98FC6F0B346AAEAE2D0F9A0C",
      "data_hash": "E41848E603B37A0527759A10372C9B97DCA45B1EBCFCB3C9ABCC06AAC82FD254",
      "validators_hash": "4507ECDC1FCB92A824F0DC3082D0A528379051D78F682DE0ECD3BAE185654FE2",
      "next_validators_hash": "4507ECDC1FCB92A824F0DC3082D0A528379051D78F682DE0ECD3BAE185654FE2",
      "consensus_hash": "048091BC7DDC283F77BFBF91D73C44DA58C3DF8A9CBC867405D8B7F3DAADA22F",
      "app_hash": "679BCBCFCB71F63E92BBCFE558C51BDD5BC4374163C2D228DFEAB3900B1F2030",
      "last_results_hash": "",
      "evidence_hash": "",
      "proposer_address": "510082967DFA7DEBA11267B26A6318D07A457B48"
    }
  }]
}
```








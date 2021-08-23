---
tags: Substrate 进阶课, batch-02
---

# Substrate 进阶课第 3 讲 II

## 大纲

- Polkadot-JS API
- FRAME collective 模块讲解
- FRAME membership 模块讲解

## 讲在开始之前

- 授人以鱼，不如授之以渔
- 会讲如何在各文档之间穿梭，这是学用 Substrate 最重要的技能

Substrate/Polkadot-JS 文档：

- 主要：[substrate.dev](https://substrate.dev)
  - [教程 tutorials](https://substrate.dev/tutorials)
  - [基础知识 knowledge base](https://substrate.dev/docs/en/)
  - [进阶菜谱 Recipes](https://substrate.dev/recipes/)
  - [how-to 手冊](https://substrate.dev/substrate-how-to-guides/)
  - [API 文档 Rustdocs](https://substrate.dev/rustdocs/)

- [Polkadot wiki](https://wiki.polkadot.network/)
  - 纪录着 Polkadot 及 Kusama 网络的基础知识及网络行为

- [Polkadot JS 文档](https://polkadot.js.org/docs/)

## Polkadot-js API

1. 首先各位要先運行 [Substrate Node Template **v3.0.0+monthly-2021-08** 版](https://github.com/substrate-developer-hub/substrate-node-template/tree/v3.0.0+monthly-2021-08)

2. 运行随本目录的 [`js-component`](./js-component)

### 連接到 Substrate 節點

```javascript
const { ApiPromise, WsProvider } = require('@polkadot/api');

// Construct
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
// 如没有运行 node-template，也可試连到波卡主網上： `wss://rpc.polkadot.io`.
const api = await ApiPromise.create({ provider: wsProvider });
```

### 读取链上元数据 (metadata)

```javascript
const { magicNumber, metadata } = await api.rpc.state.getMetadata();

console.log("Magic number: " + magicNumber);
console.log("Metadata: " + metadata.raw);
```

为什么这个重要？因为你能知道整个 链提供了什么外部交易给客户端使用

[TODO: check the following tpl still work]

```javascript
{
  magicNumber: 1635018093,
  metadata: {
    V13: {
      modules: [
        // { ... }
        {
          "name": "TemplateModule",
          "storage": {
            "prefix": "TemplateModule",
            "items": [
              {
                "name": "Something",
                "modifier": "Optional",
                "type": {
                  "plain": "u32"
                },
                "fallback": "0x00",
                "docs": []
              }
            ]
          },
          "calls": [
            {
              "name": "do_something",
              "args": [
                {
                  "name": "something",
                  "type": "u32"
                }
              ],
              "docs": [
                " An example dispatchable that takes a singles value as a parameter, writes the value to",
                " storage and emits an event. This function must be dispatched by a signed extrinsic."
              ]
            },
            {
              "name": "cause_error",
              "args": [],
              "docs": [
                " An example dispatchable that may throw a custom error."
              ]
            }
          ],
          "events": [
            {
              "name": "SomethingStored",
              "args": [
                "u32",
                "AccountId"
              ],
              "docs": [
                " Event documentation should end with an array that provides descriptive names for event",
                " parameters. [something, who]"
              ]
            }
          ],
          "constants": [],
          "errors": [
            {
              "name": "NoneValue",
              "docs": [
                " Error names should be descriptive."
              ]
            },
            {
              "name": "StorageOverflow",
              "docs": [
                " Errors should have helpful documentation associated with them."
              ]
            }
          ],
          "index": 8
        }
      ],
      "extrinsic": {
        "version": 4,
        "signedExtensions": [
          "CheckSpecVersion",
          "CheckTxVersion",
          "CheckGenesis",
          "CheckMortality",
          "CheckNonce",
          "CheckWeight",
          "ChargeTransactionPayment"
        ]
      }
    }
  }
}
```

- metadata 包含了所有 pallets (即 module 裡面的內容)，每个 pallet 的名稱，記錄着 storage, calls, events, constants, errors
- 讀取這個鏈的 metadata, 就會知道這 Substrate 鏈提供了什麼接口可供調用。
- Polkadot-JS API 也是透過讀取這數據構建出 api.[tx, consts, query].* 的接口。

詳細可看這裡: https://substrate.dev/docs/en/knowledgebase/runtime/metadata

### 基礎

- `api.tx.<pallet>.<call>` 来发送外部交易 (extrinsics)
- `api.consts.<pallet>.<const>` 来拿取 pallet 常数
- `api.query.<pallet>.<name>` 来读取 pallet 存储


### 基礎：读取某个 pallet 的常量

```javascript
// api.consts.<pallet 名称>.<常量名称>
const main = async() => {
  const existentialDeposit = await api.consts.balances.existentialDeposit
}
```

### 基礎：读取某个 pallet 的存储内容

```javascript
// api.query.<pallet 名称>.<存储名称>
// 比如:
const main = async() => {
  const acct = await api.query.system.account(alice.address);
}
```

### 基礎：發送交易

```javascript
await api.tx.balances.transfer(dest.address, amt)
  .signAndSend(src, res => {
    console.log(`Tx status: ${res.status}`);
  });
```

### 批量查询及订阅

1. **同时发多个查询**

  可同时发多个查询，而不是一条一条发

  ```javascript
  // Subscribe to balance changes for 2 accounts, ADDR1 & ADDR2 (already defined)
  const unsub = await api.query.system.account.multi([ADDR1, ADDR2], (balances) => {
   const [{ data: balance1 }, { data: balance2 }] = balances;

   console.log(`The balances are ${balance1.free} and ${balance2.free}`);
  });
  ```

   也可同时发送多个不同类型查询

   ```javascript
   // Subscribe to the timestamp, our index and balance
   const unsub = await api.queryMulti([
     api.query.timestamp.now,
     [api.query.system.account, ADDR]
   ], ([now, { nonce, data: balance }]) => {
     console.log(`${now}: balance of ${balance.free} and a nonce of ${nonce}`);
   });
   ```

   以上的开发模式有两点要注意：

   - 作查询时，传入一个 回调函数 (callback)。這是個订阅函数。你在这里更新你 react 的 state 的话，就不会出现为什么链上数据改了，而前端没有更新数据的问题。

   - `unsub`：这个 `unsub` 是一个函数，用来取消这个订阅的。如果是 react/前端开发，你在 `ComponentWillUnmount()`，或 `useEffect()` 里，就会 call 这个取消订阅函数。整个模式类似以下：

   ```javascript
   useEffect(() => {
     let unsub = null;

     const asyncFetch = async () => {
       unsub = await api.query.pallet.storage(
         param,
         result => console.log(`Result: ${result}`)
       );
     };

     asyncFetch();

     return () => {
       unsub && unsub()
     }
   }, [api, keyring]);
   ```

2. **交易并订阅事件**

  ```javascript
  // Create alice (carry-over from the keyring section)
  const alice = keyring.addFromUri('//Alice');

  // Make a transfer from Alice to BOB, waiting for inclusion
  const unsub = await api.tx.balances
   .transfer(BOB, 12345)
   .signAndSend(alice, (result) => {
     console.log(`Current status is ${result.status}`);

     if (result.status.isInBlock) {
       console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
     } else if (result.status.isFinalized) {
       console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
       unsub();
     }
   });
  ```

### 设定自订义类型

如果你看到有以下错误信息，

```
Cannot construct unknown type ...
```

如下圖：

![unknown type 报错](https://i.imgur.com/nwl3cTy.jpg)

那就说明你的 Substrate 链有一自定义类型，但 polakdot-JS API 不知道怎么解释它。需要做的是在 Polkadot-JS API 连入 Substrate 节点时，定义出该类型。如下面的 `kittyIndex`。

```javascript
const api = await ApiPromise.create({
  provider: wsProvider,
  types: {
    KittyIndex: 'u64'
  }
});
```


在用 Polkadot-JS App 時，則可在 Settings > Developer 把自定義的類型 JSON 放到這裡來。

詳情参看：https://polkadot.js.org/docs/api/start/types.extend

### keyring 钥匙圈

```javascript
// Import
const { Keyring } = require('@polkadot/keyring');
```

- 會有一組 mnemonics 來生成一個 "钥匙圈"。
- "钥匙圈" 可生出許多個 keypair 钥匙对，即公钥-私钥對。
- 这个钥匙对用来你所作的交易签名的。
- 你用你的私钥对一个交易 (可理解为一组信息，一堆 bytes) 進行签名。其他人可用你的公钥来验证这个交易为你用私钥签署的
- 签名的方法 polkadot-js API 支持：
  - ed25519
  - sr25519
  - ecdsa
  - 及 ethereum

- 而同一对钥匙对，会因应不同的网络，有着不同的網絡前䮕, network prefix 放在該公钥前，而生成出不同的帐号 (AccountID)。也就是说同一对钥匙对，在 Substrate 网络裡是一个 AccountID, 在 Polkadot 网络则會显示为另一组 AccountID, 在 Kusama 又是另一个。

```javascript
import { Keyring } from '@polkadot/keyring';
// create a keyring with some non-default values specified
const keyring = new Keyring();
```

> 小窍门： 你可访问 polkadot-js App, Developer > Javascript 内，可再加 debugger 与里面的对象物件互动。

这样，默认生成出来是用 `ed25519` 签名法，及为 Substrate 网络的帐号。

```javascript
const keyring = new Keyring({ type: 'sr25519', ss58Format: 2 });
```

这样，默认生成的出来是用 `sr25519` 签名法，及为 Kusama 网络的帐号。

ss58Format:

- `0`: Polkadot 网络
- `2`: Kusama 网络
- `42`: 一般 Substrate 网络

然后，就可这样加一个帐号：

```javascript
const mnemonic = mnemonicGenerate();

// create & add the pair to the keyring with the type and some additional
// metadata specified
const pair = keyring.addFromUri(mnemonic, { name: 'first pair' });
```

最后，拿着这个帐号，你就可对一个交易作签名：

```javascript
const txHash = await api.tx.balances
  .transfer(BOB, 12345)
  .signAndSend(alice);
```

参考:

- [SS58 地址格式](https://github.com/paritytech/substrate/wiki/External-Address-Format-(SS58))
- [Polkadot Accounts](https://wiki.polkadot.network/docs/en/learn-accounts)
- [ecdsa 签名法](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)

## 作业

前端基于 [kitties-course 已有前端](https://github.com/SubstrateCourse/kitties-hw) 加以下 UX 及功能。这部份共 10 分:

1. 能创建一个毛孩 (**3 分**)
2. 每一个毛孩展示成一张卡片，並显示是不是属于你的 (**4 分**)
3. 可以转让毛孩给另一位用户 (**3 分**)

现在前端展示：

<iframe frameBorder='0' width='640' height='360' webkitallowfullscreen mozallowfullscreen allowfullscreen src="https://www.awesomescreenshot.com/embed?id=2196893&shareKey=7749c0f9101a5791240bda8a391a1ce9"></iframe>

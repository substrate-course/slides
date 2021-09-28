# 第 6 课 - 如何使用 polkadot-js app/api

## 开始之前

学会如何在各文档之间穿梭，这是学用 Substrate 最重要的技能

Substrate/Polkadot-JS 文档：

  - 主要：[substrate.dev](https://substrate.dev/)
    - 教程 tutorials
    - 基础知识 knowledge base
    - 进阶菜谱 Recipes
    - API 文档 Rustdocs

  - [Polkadot Wiki](https://wiki.polkadot.network) / [Kusama Guide](https://guide.kusama.network/docs/en/kusama-index)

    纪录着 Polkadot 及 Kusama 网络的基础知识及网络行为

  - [Polkadot JS 文档](https://polkadot.js.org/docs/)

## 介绍

这一节，我们会先介绍 Polkadot-JS Extension，接着讲 Polkadot-JS App。它是 Polkadot 网络 (及所有基于 Substrate 开发的网络) 的官方前端，可作帐号 (钱包) 管理，转帐，也支持在 Polkadot 及 Kusama 网络上参与去中心化治理及投票，最后，也是开发者们的朋友，可设置连到本机运行的 Substrate 节点，加入自订义变量类型，以及提供一个 js 环境测试可简单篏入 js 代码读写 Substrate 网络的资讯。

接着，我们讲 Polkadot-JS API, 这是一套官方 javascript 的 SDK. 可怎样用这个 SDK 开发出你们自己的 javascript 应用 / 组件来连接到 Substrate 网络来。

以下所有项目都能从 https://polkadot.js.org/ 访问到

## Polkadot-JS Extension

https://polkadot.js.org/extension/

- 可新增帐号 (**展示**)
- 创建密码保护的延伸帐号 (**展示**)

## Polkadot-JS App 介绍

有两个版本，一个是在服务器上营运的，另一个是在 IPFS 上面的。

- Settings (设置) (**展示**)
  - 一般 - 语言
  - 开发者
  - 翻译

- Accounts (帐号管理) (**展示**)
  - 新增帐号
  - 多签名帐号
  - 简单的余额转帐

- Network (区块链网络资讯) (**展示**)
  - Explorer (链的资讯)
  - 某区块的资讯 (父区块，外部调用，事件)
  - 节点资讯

- Developer (开发者) (**展示**)
  - Chain States (链状态)
  - Extrinsics (发送外部调用)
  - RPC Calls (远程调用)
  - Sign & Verify (签名与核实)
  - Sudo (特殊权限)
  - Javascript (JS 环境)

- Governance (连到 Polkadot 主网)
  - Democracy
  - Council (理事会)
  - Treasury (国库)
  - Tech. Comm. (技术理事会)

## Polkadot-JS API 介绍

### 读取及订阅链上数据和常量

- 运行你本机上的 [Substrate Node Template](https://github.com/substrate-developer-hub/substrate-node-template)

- 运行随本目录的 [`js-component`](./js-component)

- 先连到自订义的节点上。

    ```javascript
    const { ApiPromise, WsProvider } = require('@polkadot/api');

    // Construct
    const wsProvider = new WsProvider('wss://rpc.polkadot.io');
    // 如有已在运行 node-template 可连到 `ws://127.0.0.1:9944`.
    const api = await ApiPromise.create({ provider: wsProvider });
    ```

- 读取某个 pallet 的常量：

  ```javascript
  api.consts.<pallet 名称>.<常量名称>
  // 比如:
  const existentialDeposit = await api.consts.balances.existentialDeposit
  ```

- 读取某个模塊的存储内容：

  ```javascript
  api.query.<pallet 名称>.<存储名称>
  // 比如:
  api.query.system.account(alice.address);
  ```

- 发送交易

  ```javascript
  await api.tx.balances.transfer(dest.address, amt)
    .signAndSend(src, res => {
      console.log(`Tx status: ${res.status}`);
    });
  ```

- 订阅事件

  ```javascript
  const unsub = await api.query.system.account(alice.address, aliceAcct => {
    ...
  });
  ```

- 读取链上 metadata

  ```javascript
  const metadata = await api.rpc.state.getMetadata();
  ```

## 作业

1. 访问 Polkadot-JS App 的网址是什么？

  ```
  [ ] https://polkadot.com
  [ ] https://polkadot.js.com/apps
  [ ] https://polkadot.js.org/apps
  [ ] https://polkadot.js/org/apps
  ```

2. Polkadot-JS App 和 Polkadot-JS API 的区别是什么? (把适用的全钩上)

  ```
  [ ] 名字上一看就有不同，一个尾缀是 App,一个尾缀是 API，你瞎了吗？
  [ ] Polkadot-JS App 提供了官方的 JS 库连去 Substrate 网络，而 Polkadot-JS API 则是官方的前端与 Substrate 网络 交互。
  [ ] Polkadot-JS App 是官方的前端与 Substrate 网络交互，而 Polkadot-JS API 则提供了官方的 JS 库连去 Substrate 网络。
  [ ] Polkadot-JS App 是官方的 Polkadot 钱包，而 Polkadot-JS API 则提供了官方的 JS 库连去 Substrate 网络。
  [ ] Polkadot-JS App 是官方的 Polkadot 钱包，而 Polkadot-JS API 则提供了官方的 JS 库连去现在主流的区块链网络 (Substrate，以太坊，比特币特)。
  ```

3. 你可以在 Polkadot-JS App 内做什么操作? (把适用的全勾上)

  ```
  [ ] 查看 Substrate 网络 区块信息
  [ ] 对 Substrate 网络作出交易 (Extrinsics)
  [ ] 有一个 javascript 编辑器，可对 Substrate 网络写出基础 javascript 与之互动
  [ ] 是 Substrate 的水笼头 (faucet), 可取得小额 Substrate 的代币
  [ ] 可以对一个信息以某个帐号作签名
  ```

4. 以下哪些生产环境的网络 (LIVE NETWORK) 是 Polkadot-JS App 里默认有支持的？*Checkboxes (把适用的全勾上)

  ```
  [ ] Kusama
  [ ] Acala
  [ ] Kulupu
  [ ] Centrifuge
  [ ] ChainX
  ```

5. 如果在 Substrate 端加了自定义类型，我们在 Polkadot-JS App 里需要作什么才能支持连到这个 Substrate 节点？

  ```
  [ ] 在 Setting 里, Metadata tab 里，加自定义的 JSON 对象。
  [ ] 在 Setting 里, Developer tab 里，加自定义的 XML 对象。
  [ ] 在 Setting 里, Developer tab 里，加自定义的 JSON 对象。
  [ ] 在 Toolbox 里, Sign message tab 里，先发一个签了名的信息作核实。
  [ ] 在 Toolbox 里, Verify signature tab 里，先发一个签了名的信息作核实。
  ```

6. 在 Polkadot-JS API 里，数字默认是用哪个类型代表？

  ```
  [ ] JS 里默认的数字类型
  [ ] 字付串
  [ ] [bn.js](https://github.com/indutny/bn.js/)
  [ ] [bignumber.js](https://github.com/MikeMcl/bignumber.js/)
  [ ] [decimal.js](https://github.com/MikeMcl/decimal.js/)
  ```

7. 我要查询 Substrate 链上的存储变量,并订阅它的变更，应该用以下哪个方法？

  ```
  [ ] `const val = await api.query.my_pallet.storage`
  [ ] `const unsub = await api.query.my_pallet.storage(value => {...})`
  [ ] `const val = await api.consts.my_pallet.const`
  [ ] `const val = await api.tx.my_pallet.tx().signAndSend()`
  [ ] `const val = await api.tx.my_pallet.tx().signAndSend(value => {...})`
  ```

8. 我要对 Substrate 链上发出一次交易，但 **不需要** 订阅交易处理状态，应该用以下哪个方法？

  ```
  [ ] `const val = await api.query.my_pallet.storage`
  [ ] `const unsub = await api.query.my_pallet.storage(value => {...})`
  [ ] `const val = await api.consts.my_pallet.const`
  [ ] `const val = await api.tx.my_pallet.tx().signAndSend()`
  [ ] `const val = await api.tx.my_pallet.tx().signAndSend(value => {...})`
  ```

9. 现在在 Github 上的 [Substrate repo](https://github.com/paritytech/substrate) 约有多少用户给它打了星？选个最接近的。

  ```
  [ ] 500
  [ ] 1,000
  [ ] 3,000
  [ ] 5,000
  [ ] 7,500
  ```

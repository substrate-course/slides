# 第 6 课 - 如何使用 polkadot-js app/api

## 开始之前

学会如何在各文档之间穿梭，这是学用 Substrate 最重要的技能

Substrate/Polkadot-JS 文档：

  - 主要：[substrate.dev](https://substrate.dev/)
    - 教程 tutorials
    - 基础知识 knowledge base
    - 进阶菜谱 Recipes
    - API 文档 Rustdocs

  - [Polkadot wiki](https://wiki.polkadot.network)

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

- 等到状态 `isReady()`

- 读取某个 pallet 的常量：

  ```javascript
  api.consts.<pallet 名称>.<常量名称>
  // 比如:
  api.consts.balances.existentialDeposit
  ```

- 读取某个存储内容：

  ```javascript
  api.query.<pallet 名称>.<存储名称>
  // 比如:
  api.query.balances.existentialDeposit
  ```

- 发送交易
- 读取链上 metadata
- 订阅事件

## 作业

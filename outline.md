[Chinese](#Substrate-区块链应用开发), [English](#Substrate-blockchain-application-development)

## Substrate 区块链应用开发

1. Substrate & Polkadot 简介 
    - 为什么学习 Substrate
    - Polkadot 特点和生态
    - 搭建开发环境
        - 命令行参数
        - Single Node Development Chain
        - Multi-Node Local Testnet
    - Node template 代码导读
    - 作业

2. Proof of Existence 教程（一）
    - Runtime Macros 介绍
    - Storage Data Type
    - 模块功能开发
    - 作业

3. Proof of Existence 教程（二）
    - Transaction Weight & Fees
    - 单元测试
    - 编写UI
    - FRAME 基础功能模块介绍
        - timestamp
        - system
        - utility
        - transaction-payment
    - 作业

4. polkadot-js app/api 如何使用 
    - Polkadot-JS App 详细介绍
    - 介绍和安装
    - 常用功能讲解
        - 读取链上数据和常量
        - 订阅链上数据的变化
        - 发送交易
        - 账户管理和签名
        - 订阅事件
        - 注册自定义类型
    - 使用Frontend template 打造React应用
    - 作业

5. Substrate Kitties 教程（一）
    - Metadata元数据介绍
    - SCALE codec 介绍
    - 模块功能开发-1
    - 单元测试-1
    - 作业

6. Substrate Kitties 教程（二）
    - 链上升级和数据迁移
    - 模块功能开发-2
    - 单元测试-2
    - FRAME 资产相关模块介绍
        - balances
        - assets
        - generic-asset
    - 作业

7. Substrate Kitties 教程（三）
    - 模块间功能复用
    - 编写UI
    - FRAME 治理相关模块介绍
        - sudo
        - democracy
        - collective
        - treasury
        - elections-phragmen
        - membership
    - 作业

8. Off-chain Worker 教程（一）
    - Substrate 密码学
    - 介绍及代码前期衔接
        - Send Signed Transaction
        - Send Unsigned Transaction
        - Unsigned transaction with signed payload
    - 单元测试
    - 作业

9. Off-chain Worker 教程（二）
    - 发送 HTTP 请求及 JSON 解析
    - off-chain worker 存储数据库
    - FRAME 权益证明相关模块介绍:
        - im-online
        - staking
        - session
    - 作业

10. Smart Contract 教程（一）
    - FRAME 智能合约相关模块介绍
        - contracts
    - 使用 ink! 编写智能合约
    - 作业

11. Smart Contract 教程（二）
    - 运行 Solidity 智能合约
    - 社交相关模块介绍
        - identity
        - recovery
        - society
    - 作业

12. 测试和上线 
    - Benchmark 和 Runtime 参数
    - 安全性验证
    - 通证经济模型
    - Chain Specification
    - 配置公开测试网
    - 总结和展望

## Substrate blockchain application development

1. Substrate & Polkadot Overview 
    - Why Learn Substrate
    - The Feature and Ecosystem of Polkadot
    - Setup Development Environment
        - CLI parameters
        - Single Node Development Chain
        - Multi-Node Local Testnet
    - Node template code guidance
    - Homework

2. Proof of Existence tutorial（1）
    - Introduction of Runtime Macros
    - Storage Data Type
    - Development of Module Functionality
    - Homework

3. Proof of Existence Tutorial（2）
    - Transaction Weight & Fees
    - Unit Test
    - Develop UI
    - Introduction of FRAME Fundamental Modules
        - timestamp
        - system
        - utility
        - transaction-payment
    - Homework

4. How to Use Polkadot-js/api 
    - Introduction and Setup
    - Explanation of common functions
        - Query on-chain storage and constants
        - Subscribe the change of on-chain data
        - Send a transaction
        - Account management and signing
        - Subscribe events
        - Register custom type
    - Build a React app with Frontend template
    - Interactive with node
        - HTTP/HTTPS
        - WebSocket
    - Homework

5. Substrate Kitties Tutorial（1）
    - Introduction on Metadata struct
    - SCALE codec (data encoding/decoding)
    - Pallet feature development - 1
    - Unit Test - 1
    - Exercise

6. Substrate Kitties Tutorial（2）
    - Runtime upgrade and data migration
    - Pallet feature development - 2
    - Unit Test - 2
    - FRAME: introduction to asset-related pallets:
        - balances
        - assets
        - generic-asset
    - Exercise

7. Substrate Kitties Tutorial（3）
    - Code reuse across pallets
    - UI programming
    - FRAME: introduction to governance-related pallets:
        - sudo
        - democracy
        - collective
        - treasury
        - elections-phragmen
        - membership
    - Exercise

8. Off-chain Worker Tutorial（1）
    - Cryptography in Substrate
    - Off-chain work introduction and Setup
        - Send Signed Transaction
        - Send Unsigned Transaction
        - Unsigned transaction with signed payload
    - Unit Test for off-chain worker
    - Exercise

9. Off-chain Worker Tutorial（2）
    - Sending HTTP requests and JSON parsing
    - off-chain worker local storage
    - FRAME: introduction on staking-related pallets:
        - im-online
        - staking
        - session
    - Exercise

10. Smart Contract Tutorial（1) 
    - FRAME: introduction on smart contract-related pallets
        - contracts
        - evm
    - Writing Smart Contract in ink!
    - Exercise

11. Smart Contract Tutorial（2) 
    - Running Solidity Smart Contract
    - FRAME: introduction on society-related pallets
        - identity
        - recovery
        - society
    - Exercise

12. Testing and Deployment 
    - Benchmark and Runtime Configuration
    - Security Concern
    - Tokenomics
    - Chain Specification
    - Deploy public testnet
    - Course Summary
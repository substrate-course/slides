> 注 #1：
> 
> `main` 分支要用 rust stable, `rustc 1.49.0 (e1884a8e3 2020-12-29)` 来编译。  
> 而 `rust-nightly-2020-10-05` 分支则要用 rust nightly, `rustc 1.49.0-nightly (beb5ae474 2020-10-04)` 来编译。  
> 请学员们根据自身编译环境选择。
> 谢谢 @腾龙-开发-北京 的改良建议

---

# Substrate 进阶课第四讲 - 链下工作机 (Off-chain Worker)

<!-- MarkdownTOC autolink="true" -->

- [Substrate 密码学](#substrate-%E5%AF%86%E7%A0%81%E5%AD%A6)
  - [哈希键生成方法](#%E5%93%88%E5%B8%8C%E9%94%AE%E7%94%9F%E6%88%90%E6%96%B9%E6%B3%95)
  - [钥匙对生成及签名法](#%E9%92%A5%E5%8C%99%E5%AF%B9%E7%94%9F%E6%88%90%E5%8F%8A%E7%AD%BE%E5%90%8D%E6%B3%95)
- [链下工作机 off-chain worker \(ocw\)](#%E9%93%BE%E4%B8%8B%E5%B7%A5%E4%BD%9C%E6%9C%BA-off-chain-worker-ocw)
  - [什么是 ocw?](#%E4%BB%80%E4%B9%88%E6%98%AF-ocw)
  - [使用 ocw](#%E4%BD%BF%E7%94%A8-ocw)
    - [签名交易](#%E7%AD%BE%E5%90%8D%E4%BA%A4%E6%98%93)
    - [不具签名交易](#%E4%B8%8D%E5%85%B7%E7%AD%BE%E5%90%8D%E4%BA%A4%E6%98%93)
    - [不签名但具签名信息的交易](#%E4%B8%8D%E7%AD%BE%E5%90%8D%E4%BD%86%E5%85%B7%E7%AD%BE%E5%90%8D%E4%BF%A1%E6%81%AF%E7%9A%84%E4%BA%A4%E6%98%93)
    - [发 HTTP 请求](#%E5%8F%91-http-%E8%AF%B7%E6%B1%82)
    - [解析 JSON](#%E8%A7%A3%E6%9E%90-json)
    - [ocw 自己链下的独立存储](#ocw-%E8%87%AA%E5%B7%B1%E9%93%BE%E4%B8%8B%E7%9A%84%E7%8B%AC%E7%AB%8B%E5%AD%98%E5%82%A8)
- [Pallet 讲解: `pallet-im-online`](#pallet-%E8%AE%B2%E8%A7%A3-pallet-im-online)
- [作业](#%E4%BD%9C%E4%B8%9A)

<!-- /MarkdownTOC -->

## Substrate 密码学

- 还是先过一下理论作铺垫
- Substrate 里其中两处用到密码学的地方是它的 **哈希方法** 和 **钥匙对的生成和使用**。

### 哈希键生成方法

```rust
pub OwnedKitties get(fn owned_kitties): map hasher(blake2_128_concat)
  (T::AccountId, Option<T::KittyIndex>) => Option<KittyLinkedItem<T>>;
```

- 这个 `blake2_128_concat` 是用作从后面的参数，指定怎样生成成键 (key) 的方法。它是一个密码学的生成方法。

这些方法需要有三个特质：

![hash-func.jpg](./assets/hash-func.jpg)

- 不容易从观察 **生成后结果** 倒推回 **生成前参数**。
- 如果 **生成前参数** 不一样，**生成后结果** 也不容易有重覆。但如果生成前是同一个参数，则要生成出一样的结果。
- **生成前参数** 如果有一丁点的改变，也会导致到 **生成后结果** 很大的改变。

而现在 `map` 键生成的方法支持:

1. `identity`: 对参数不作加密处理，直接拿作键值用。通常这是用在键参数不是用户控制的值上的。

2. `twox_64_concat`: 优点是非常的快 及支持 map 可遍历它的所有键，缺点是密码学上 "不是绝对安全"。

3. `blake2_128_concat`: 优点是密码学上相对安全，也支持该 map 可遍历它的所有键，缺点是需要一定计算量，相较 #2 较慢。

如果你们不知道选谁最合适，就选 #3 吧 😁

参考：

- https://substrate.dev/rustdocs/v2.0.0/frame_support/macro.decl_storage.html
- https://substrate.dev/docs/en/knowledgebase/advanced/cryptography
- https://wiki.polkadot.network/docs/en/learn-cryptography

### 钥匙对生成及签名法

- 在 Substrate, 所有钥匙对的实例都得实践 [`Pair` trait](https://substrate.dev/rustdocs/v2.0.0/sp_core/crypto/trait.Pair.html)

Substrate 支持三种钥匙生成及签名法

1. `ECDSA`: 基于 secp256k1 曲线的 `ECDSA` 签名算法

  - Bitcoin 和 Ethereum 都是用这钥匙生成及签名法
  - 参考 [secp256k1 曲线](https://en.bitcoin.it/wiki/Secp256k1)
  - 参考 [ECDSA 签名算法](https://en.wikipedia.org/wiki/Elliptic_Curve_Digital_Signature_Algorithm)

2. `Ed25519`: 基于 25519 曲线 (Curve25519) 的 `EdDSA` 签名算法

  - 参考 [25519 曲线](https://en.wikipedia.org/wiki/Curve25519)
  - 参考 [Ed25519](https://en.wikipedia.org/wiki/EdDSA#Ed25519)

3. `SR25519`: 基于受过 Ristretto 压缩法 (那个 `R`) 的 25519 曲线 的 Schnorrkel 签名算法 (那个 `S`)

  ![sr25519 插图](./assets/sr25519-algo.png)

  - 好处 1: 基于 `Ed25519` 再作了一些安全性的改良。把 25519 曲线的一些隐患解决掉。也是 Substrate 默认开帐号时用的方法
  - 好处 2: 有更好的 key 的 路径支持 (hierarchical deterministic key derivations)
  - 好处 3:  本身支持集成多签名
  - 参考 [Polkadot wiki: sr25519](https://wiki.polkadot.network/docs/en/learn-keys#what-is-sr25519-and-where-did-it-come-from)
  - 参考 [Polkadot wiki: keypairs](https://wiki.polkadot.network/docs/en/learn-cryptography#keypairs-and-signing)

## 链下工作机 off-chain worker (ocw)

### 什么是 ocw?

![off-chain-workers-v2](./assets/off-chain-workers-v2.png)

- 链上 runtime 逻辑有以下限制：

  - 所有计算不能占时太长，不然影响出块时间
  - 不能做没有绝对结果 (deterministic) 的操作。比如说发一个 http 请求。因为：1）有时可能会失败。2) 返回的结果不会时时都一样。
  - 最好不要占太多链上存储。因为每个数据都得重覆一篇存在每个节点上。

- 所以衍生出链下工作机 (off-chain worker), 简称 ocw.
- ocw 有以下特质：
  - 它在另一个（链下环境）运行，运行不影响出块
  - 链下工作机能读到链上存储的数据，但不能直接写到链上存储。
  - 它有一个专属的存储位置。存储在这里，只供这节点的所有链下工作机进程读写。
  - 同一时间可有多个链下工作机进程在跑着

    ![multiple-ocws.png](./assets/multiple-ocws.png)

- 它适合作什么？
  - 计算量大的工作
  - 没有绝对结果的操作
  - 有一些需要缓存数据的计算 (利用上 ocw 的单节点存储)

### 使用 ocw

以下开始进入编程环节，讲代码。大家可 git clone [advance-lecture-04-ocw](https://github.com/SubstrateCourse/advance-lecture-04-ocw). 跟着一起跑。我也是讲里面的内容。成功编译后跑起来会是这样的:

https://www.awesomescreenshot.com/video/2423609?key=a190e0063aab700d8354e78f2d5db9a9

首先从 `pallets/ocw-demo/src` 谈起。

触发 ocw，一个区块生成 (称作 block import) 有三个阶段

- 区块初始化 (block initialization)
- 跑链上逻辑
- 区块最终化 (block finalization)

参考 [rustdoc](https://substrate.dev/rustdocs/v2.0.0/frame_system/enum.Phase.html)

你们定义的 pallet 都有 [OnInitialize](https://substrate.dev/rustdocs/v2.0.0/frame_support/traits/trait.OnInitialize.html), 及 [OnFinalize]((https://substrate.dev/rustdocs/v2.0.0/frame_support/traits/trait.OnFinalize.html)) 函数可供设定回调

完成一次区块生成后，就会调用以下 ocw 入口。

```rust
fn offchain_worker(block_number: T::BlockNumber) {
  debug::info!("Entering off-chain worker");
  // ...
}
```

接下来我们可用三种交易方法把计算结果写回链上：

  1. 签名交易
  2. 不签名交易
  3. 不签名交易但有签名数据

#### 签名交易

主要看代码里： `Self::offchain_signed_tx(block_number)`

1. 先从新定义一个用来签名的钥匙

    ```rust
    pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

    pub mod crypto {
      use crate::KEY_TYPE;
      use sp_runtime::app_crypto::{app_crypto, sr25519};
      // -- snip --
      app_crypto!(sr25519, KEY_TYPE);
    }
    ```

2. 你的 pallet Trait 也需要加多一个约束 `CreateSignedTransaction`:

    ```rust
    pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
      /// The identifier type for an offchain worker.
      type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
      /// The overarching dispatch call type.
      type Call: From<Call<Self>>;
      /// The overarching event type.
      type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    }
    ```

3. 看看在 runtime 里是如何实现这个 pallet 的：

    `runtimes/src/lib.rs`

    ```rust
    impl pallet_ocw_demo::Trait for Runtime {
      type AuthorityId = pallet_ocw_demo::crypto::TestAuthId;
      type Call = Call;
      type Event = Event;
    }

    impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
    where
      Call: From<LocalCall>,
    {
      fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        public: <Signature as traits::Verify>::Signer,
        account: AccountId,
        index: Index,
      ) -> Option<(Call, <UncheckedExtrinsic as traits::Extrinsic>::SignaturePayload)> {
        let period = BlockHashCount::get() as u64;
        let current_block = System::block_number()
          .saturated_into::<u64>()
          .saturating_sub(1);
        let tip = 0;
        let extra: SignedExtra = (
          frame_system::CheckSpecVersion::<Runtime>::new(),
          frame_system::CheckTxVersion::<Runtime>::new(),
          frame_system::CheckGenesis::<Runtime>::new(),
          frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
          frame_system::CheckNonce::<Runtime>::from(index),
          frame_system::CheckWeight::<Runtime>::new(),
          pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
        );

        let raw_payload = SignedPayload::new(call, extra)
          .map_err(|e| {
            debug::warn!("SignedPayload error: {:?}", e);
          })
          .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        let address = account;
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (multiaddress::MultiAddress::Id(address), signature.into(), extra)))
      }
    }

    impl frame_system::offchain::SigningTypes for Runtime {
      type Public = <Signature as traits::Verify>::Signer;
      type Signature = Signature;
    }

    impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
    where
      Call: From<C>,
    {
      type OverarchingCall = Call;
      type Extrinsic = UncheckedExtrinsic;
    }
    ```

4. 在 `node/src/service.rs` 加 keystore 一段

    ```rust
    keystore.write().insert_ephemeral_from_seed_by_type::<runtime::pallet_ocw_demo::crypto::Pair>(
      "//Alice", runtime::pallet_ocw_demo::KEY_TYPE
    ).expect("Creating key with account Alice should succeed.");
    ```

5. 接下来看 `fn offchain_signed_tx` 内的函数

    ```rust
    fn offchain_signed_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
      // We retrieve a signer and check if it is valid.
      //   Since this pallet only has one key in the keystore. We use `any_account()1 to
      //   retrieve it. If there are multiple keys and we want to pinpoint it, `with_filter()` can be chained,
      //   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.Signer.html
      let signer = Signer::<T, T::AuthorityId>::any_account();

      // Translating the current block number to number and submit it on-chain
      let number: u32 = block_number.try_into().unwrap_or(0);

      // `result` is in the type of `Option<(Account<T>, Result<(), ()>)>`. It is:
      //   - `None`: no account is available for sending transaction
      //   - `Some((account, Ok(())))`: transaction is successfully sent
      //   - `Some((account, Err(())))`: error occured when sending the transaction
      let result = signer.send_signed_transaction(|_acct|
        // This is the on-chain function
        Call::submit_number_signed(number)
      );

      // Display error if the signed tx fails.
      if let Some((acc, res)) = result {
        if res.is_err() {
          debug::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
          return Err(<Error<T>>::OffchainSignedTxError);
        }
        // Transaction is sent successfully
        return Ok(());
      }

      // The case of `None`: no account is available for sending
      debug::error!("No local account available");
      Err(<Error<T>>::NoLocalAcctForSigning)
    }
    ```

#### 不具签名交易

1. 调用 `SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction`

    看 `pallets/ocw-demo/src/lib.rs`

    ```rust
    fn offchain_unsigned_tx(block_number: T::BlockNumber) -> Result<(), Error<T>> {
      let number: u32 = block_number.try_into().unwrap_or(0);
      let call = Call::submit_number_unsigned(number);

      // `submit_unsigned_transaction` returns a type of `Result<(), ()>`
      //   ref: https://substrate.dev/rustdocs/v2.0.0/frame_system/offchain/struct.SubmitTransaction.html#method.submit_unsigned_transaction
      SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
        .map_err(|_| {
          debug::error!("Failed in offchain_unsigned_tx");
          <Error<T>>::OffchainUnsignedTxError
        })
    }
    ```

2. 默认不具签名的交易是会被拒绝的。所以需要一个函数定明我们的自定义核对逻辑并批准这函数通过。

    看 `pallets/ocw-demo/src/lib.rs`

    ```rust
    impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
      type Call = Call<T>;

      fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
        let valid_tx = |provide| ValidTransaction::with_tag_prefix("ocw-demo")
          .priority(UNSIGNED_TXS_PRIORITY)
          .and_provides([&provide])
          .longevity(3)
          .propagate(true)
          .build();

        match call {
          Call::submit_number_unsigned(_number) => valid_tx(b"submit_number_unsigned".to_vec()),
          Call::submit_number_unsigned_with_signed_payload(ref payload, ref signature) => {
            if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
              return InvalidTransaction::BadProof.into();
            }
            valid_tx(b"submit_number_unsigned_with_signed_payload".to_vec())
          },
          _ => InvalidTransaction::Call.into(),
        }
      }
    }
    ```

#### 不签名但具签名信息的交易

看 `offchain_unsigned_tx_signed_payload`

```rust
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct Payload<Public> {
  number: u32,
  public: Public
}

// ...

fn offchain_unsigned_tx_signed_payload(block_number: T::BlockNumber) -> Result<(), Error<T>> {
  // Retrieve the signer to sign the payload
  let signer = Signer::<T, T::AuthorityId>::any_account();

  let number: u32 = block_number.try_into().unwrap_or(0);

  // `send_unsigned_transaction` is returning a type of `Option<(Account<T>, Result<(), ()>)>`.
  //   Similar to `send_signed_transaction`, they account for:
  //   - `None`: no account is available for sending transaction
  //   - `Some((account, Ok(())))`: transaction is successfully sent
  //   - `Some((account, Err(())))`: error occured when sending the transaction
  if let Some((_, res)) = signer.send_unsigned_transaction(
    |acct| Payload { number, public: acct.public.clone() },
    Call::submit_number_unsigned_with_signed_payload
  ) {
    return res.map_err(|_| {
      debug::error!("Failed in offchain_unsigned_tx_signed_payload");
      <Error<T>>::OffchainUnsignedTxSignedPayloadError
    });
  }

  // The case of `None`: no account is available for sending
  debug::error!("No local account available");
  Err(<Error<T>>::NoLocalAcctForSigning)
}
```

主要我们定义了 `Payload` 这个结构体。

为什么会有 **不签名但具签名信息的交易**? 因为很多时候签名交易意味签名者需要为该交易付手续费。但有些情况你想知道该交易来源是谁，但不需要该用户付手续费。

#### 发 HTTP 请求

接下来我们从 github 那里获取 Substrate 开发者中心的数据。这要用上 http request 和 解析 JSON 的能力。

```rust
pub const HTTP_REMOTE_REQUEST: &str = "https://api.github.com/orgs/substrate-developer-hub";
pub const HTTP_HEADER_USER_AGENT: &str = "jimmychu0807";

#[derive(Deserialize, Encode, Decode, Default)]
struct GithubInfo {
  // Specify our own deserializing function to convert JSON string to vector of bytes
  #[serde(deserialize_with = "de_string_to_bytes")]
  login: Vec<u8>,
  #[serde(deserialize_with = "de_string_to_bytes")]
  blog: Vec<u8>,
  public_repos: u32,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(de)?;
  Ok(s.as_bytes().to_vec())
}

fn fetch_n_parse() -> Result<GithubInfo, Error<T>> {
  let resp_bytes = Self::fetch_from_remote().map_err(|e| {
    debug::error!("fetch_from_remote error: {:?}", e);
    <Error<T>>::HttpFetchingError
  })?;

  let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::HttpFetchingError)?;
  // Print out our fetched JSON string
  debug::info!("{}", resp_str);

  // Deserializing JSON to struct, thanks to `serde` and `serde_derive`
  let gh_info: GithubInfo =
    serde_json::from_str(&resp_str).map_err(|_| <Error<T>>::HttpFetchingError)?;
  Ok(gh_info)
}

fn fetch_from_remote() -> Result<Vec<u8>, Error<T>> {
  debug::info!("sending request to: {}", HTTP_REMOTE_REQUEST);

  // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
  let request = rt_offchain::http::Request::get(HTTP_REMOTE_REQUEST);

  // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
  let timeout = sp_io::offchain::timestamp()
    .add(rt_offchain::Duration::from_millis(FETCH_TIMEOUT_PERIOD));

  // For github API request, we also need to specify `user-agent` in http request header.
  //   See: https://developer.github.com/v3/#user-agent-required
  let pending = request
    .add_header("User-Agent", HTTP_HEADER_USER_AGENT)
    .deadline(timeout) // Setting the timeout time
    .send() // Sending the request out by the host
    .map_err(|_| <Error<T>>::HttpFetchingError)?;

  // By default, the http request is async from the runtime perspective. So we are asking the
  //   runtime to wait here.
  // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
  //   ref: https://substrate.dev/rustdocs/v2.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
  let response = pending
    .try_wait(timeout)
    .map_err(|_| <Error<T>>::HttpFetchingError)?
    .map_err(|_| <Error<T>>::HttpFetchingError)?;

  if response.code != 200 {
    debug::error!("Unexpected http request status code: {}", response.code);
    return Err(<Error<T>>::HttpFetchingError);
  }

  // Next we fully read the response body and collect it to a vector of bytes.
  Ok(response.body().collect::<Vec<u8>>())
}
```

#### 解析 JSON

- 其实解析 JSON 也不太难，用 `serde` 库就是了
- 不过 cargo 有一个问题，我们 runtime 里有 serde, 并且会编译支持 `std`, 所以现在如果在 `ocw-demo` pallet 用同一个 serde 就会自动支持 `std` （详细解释在这 [github issue](https://github.com/rust-lang/cargo/issues/4463)）。
- 所以同一个套代码，在 cargo crate 上命名为 `alt_serde`

```rust
// ref: https://serde.rs/container-attrs.html#crate
#[derive(Deserialize, Encode, Decode, Default)]
struct GithubInfo {
  // Specify our own deserializing function to convert JSON string to vector of bytes
  #[serde(deserialize_with = "de_string_to_bytes")]
  login: Vec<u8>,
  #[serde(deserialize_with = "de_string_to_bytes")]
  blog: Vec<u8>,
  public_repos: u32,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(de)?;
  Ok(s.as_bytes().to_vec())
}
```

#### ocw 自己链下的独立存储

```rust
fn fetch_github_info() -> Result<(), Error<T>> {
  // Create a reference to Local Storage value.
  // Since the local storage is common for all offchain workers, it's a good practice
  // to prepend our entry with the pallet name.
  let s_info = StorageValueRef::persistent(b"offchain-demo::gh-info");

  // Local storage is persisted and shared between runs of the offchain workers,
  // offchain workers may run concurrently. We can use the `mutate` function to
  // write a storage entry in an atomic fashion.
  //
  // With a similar API as `StorageValue` with the variables `get`, `set`, `mutate`.
  // We will likely want to use `mutate` to access
  // the storage comprehensively.
  //
  // Ref: https://substrate.dev/rustdocs/v2.0.0/sp_runtime/offchain/storage/struct.StorageValueRef.html
  if let Some(Some(gh_info)) = s_info.get::<GithubInfo>() {
    // gh-info has already been fetched. Return early.
    debug::info!("cached gh-info: {:?}", gh_info);
    return Ok(());
  }

  // Since off-chain storage can be accessed by off-chain workers from multiple runs, it is important to lock
  //   it before doing heavy computations or write operations.
  // ref: https://substrate.dev/rustdocs/v2.0.0-rc3/sp_runtime/offchain/storage_lock/index.html
  //
  // There are four ways of defining a lock:
  //   1) `new` - lock with default time and block exipration
  //   2) `with_deadline` - lock with default block but custom time expiration
  //   3) `with_block_deadline` - lock with default time but custom block expiration
  //   4) `with_block_and_time_deadline` - lock with custom time and block expiration
  // Here we choose the most custom one for demonstration purpose.
  let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(
    b"offchain-demo::lock", LOCK_BLOCK_EXPIRATION,
    rt_offchain::Duration::from_millis(LOCK_TIMEOUT_EXPIRATION)
  );

  // We try to acquire the lock here. If failed, we know the `fetch_n_parse` part inside is being
  //   executed by previous run of ocw, so the function just returns.
  // ref: https://substrate.dev/rustdocs/v2.0.0/sp_runtime/offchain/storage_lock/struct.StorageLock.html#method.try_lock
  if let Ok(_guard) = lock.try_lock() {
    match Self::fetch_n_parse() {
      Ok(gh_info) => { s_info.set(&gh_info); }
      Err(err) => { return Err(err); }
    }
  }
  Ok(())
}
```

参考 [`StorageValueRef` rustdocs](https://substrate.dev/rustdocs/v2.0.0/sp_runtime/offchain/storage/struct.StorageValueRef.html)

## Pallet 讲解: `pallet-im-online`

- 首先，打开 [rustdoc 文档](https://substrate.dev/rustdocs/v2.0.0/pallet_im_online/index.html)

- 它是作为一个 validator 发一次心跳 (heartbeat) 出去给其他 validators。证明自己在该 era 里自己是在线的。如果一个 validator 在一个 era 里一次心跳都没有，则会被视作不在线，而自己的质押也会有惩罚。

- 他的心跳是用 offchain worker 的 **不签名但具签名信息的交易** (unsigned transaction with signed payload) 来完成的。

代码：

  1. L#107 - 140: 载入这个 pallet 的签名
  2. L#153 - 228: 定义不同的结构体，和 enum 错误
  3. L#230 - 260: 该 pallet 的 `Trait` (最新 Substrate 改了名称叫 Config, 因为我们全称这个东西为 pallet configurable trait). Runtime 在实现这个 pallet 时需要实现这个 trait。
  4. L#277 - 306: pallet 的存储
  5. L#308 - 316: pallet 回传山来外部的错误信息
  6. 主要逻辑： offchain_worker 入口

    - L#372 - L#394: `fn offchain_worker`
    - L#455 - L#476: `Self::send_heartbeats`
    - L#479 - L#530: `Self::send_heartbeat`, 留意用了 `submit_unsigned_transaction`. 回调 `Call::heartbeat`

  7. L#339 - 369: 回看 `Call::heartbeat` 是做什么
  8. 也看 runtime 怎样实现 pallet_im_online, `substrate/runtime/src/lib.rs` 的 L#809 - 816

## 作业

以 `lecture-demo` 作基础，把它拷到 `assignment` 目录里来修改，最后提交这个代码库。

利用 offchain worker 取出 DOT 当前对 USD 的价格，并把写到一个 Vec 的存储里，你们自己选一种方法提交回链上，并在代码注释为什么用这种方法提交回链上最好。只保留当前最近的 10 个价格，其他价格可丢弃 （就是 Vec 的长度长到 10 后，这时再插入一个值时，要先丢弃最早的那个值）。

这个 http 请求可得到当前 DOT 价格：[https://api.coincap.io/v2/assets/polkadot](https://api.coincap.io/v2/assets/polkadot)。

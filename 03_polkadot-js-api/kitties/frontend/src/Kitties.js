import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

function Kitty (id, dna, owner) {
  this.id = id
  this.dna = dna
  this.owner = owner
}

Kitty.prototype.printInfo = function () {
  console.log(`猫咪的id为：${this.id}，猫咪的dna为：${this.dna}，猫咪的主人为：${this.owner}。`)
}

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKitties = () => {
    // TODO: 在这里调用 `api.query.kittiesModule.*` 函数去取得猫咪的信息。
    // 你需要取得：
    //   - 共有多少只猫咪
    //   - 每只猫咪的主人是谁
    //   - 每只猫咪的 DNA 是什么，用来组合出它的形态
    async function _getKittiesInfo () {
      const kittiesModule = api.query.kittiesModule

      const kittiesCountOption = await kittiesModule.kittiesCount()
      const kittiesCount = kittiesCountOption.isNone ? 0 : kittiesCountOption.unwrap().toNumber()

      // 链上猫咪总数
      console.log(`猫咪总数：${kittiesCount}`)

      if (kittiesCount === 0) return

      const kittiesInfos = new Array(kittiesCount).fill(0)

      const queries = kittiesInfos.map((_ignore, kittyIndex) => {
        return new Promise((resolve) => {
          api.queryMulti(
            [
              [kittiesModule.kitties, kittyIndex],
              [kittiesModule.owner, kittyIndex]
            ],
            ([kittyDNA, kittyOwner]) => {
              kittiesInfos[kittyIndex] = new Kitty(
                kittyIndex,
                kittyDNA.unwrap().toU8a(),
                kittyOwner.unwrap().toString()
              )
              resolve()
            }
          )
        })
      })

      await Promise.all(queries)

      kittiesInfos.forEach(_kittyInfo => _kittyInfo.printInfo())

      setKitties(kittiesInfos)
    }

    _getKittiesInfo()
  }

  const populateKitties = () => {
    // TODO: 在这里添加额外的逻辑。你需要组成这样的数组结构：
    //  ```javascript
    //  const kitties = [{
    //    id: 0,
    //    dna: ...,
    //    owner: ...
    //  }, { id: ..., dna: ..., owner: ... }]
    //  ```
    // 这个 kitties 会传入 <KittyCards/> 然后对每只猫咪进行处理
    const kitties = []
    setKitties(kitties)
  }

  useEffect(fetchKitties, [api, keyring, status])
  useEffect(populateKitties, [])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}

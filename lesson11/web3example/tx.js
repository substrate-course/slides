const Web3 = require("web3");
// praivate key of addressFrom
const privateKey = "99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342";
const addressFrom = "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b";
const addressTo = "0xB90168C8CBcd351D069ffFdA7B71cd846924d551";
const web3 = new Web3('http://127.0.0.1:9933');

const transfer = async() => {
    console.log(`Attempting to make transaction from ${addressFrom} to ${addressTo}`);

    const createTx = await web3.eth.accounts.signTransaction(
        {
            from: addressFrom,
            to: addressTo,
            value: web3.utils.toWei('100', 'ether'),
            gas: '4294967295',
        },
        privateKey
    );

    const createReceipt = await web3.eth.sendSignedTransaction(
        createTx.rawTransaction
    );
    console.log(`Transaction successful with hash ${createReceipt.transactionHash}`)

};

transfer();
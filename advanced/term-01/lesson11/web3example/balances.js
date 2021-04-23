const Web3 = require("web3");
// praivate key of addressFrom
const privateKey = "99B3C12287537E38C90A9219D4CB074A89A16E9CDB20BF85728EBD97C343E342";
const addressFrom = "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b";
const addressTo = "0xB90168C8CBcd351D069ffFdA7B71cd846924d551";
const web3 = new Web3('http://127.0.0.1:9933');

const balances = async() => {
    const balanceFrom = web3.utils.fromWei(
        await web3.eth.getBalance(addressFrom),
        'ether'
    );

    const balanceTo = await web3.utils.fromWei(
        await web3.eth.getBalance(addressTo),
        'ether'
    );

    console.log(`The balance of ${addressFrom} is: ${balanceFrom} ETH`);
    console.log(`The balance of ${addressTo} is: ${balanceTo} ETH`);
};

balances();
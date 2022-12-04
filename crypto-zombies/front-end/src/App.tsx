import { useEffect, useState } from 'react';
import { useRecoilState, useRecoilValue } from 'recoil';
import Web3 from 'web3';
import Generate from './components/Generate';
import Zombie from './components/Zombie';
import { abi, token_abi, TokenContractAddress, contractAddress, WS_API } from './contract/util';
import { InfomationZombie, NameZombie } from './recoil/zombie-detail/zombieDetail';

function App() {
  const web3 = new Web3(Web3.givenProvider);
  const myContract = new web3.eth.Contract(abi, contractAddress);
  const [address, setAddress] = useState<String>('');
  const textNameZombie = useRecoilValue(NameZombie);

  const testToken = new web3.eth.Contract(token_abi, TokenContractAddress);

  // login
  const ethereum = window.ethereum;
  console.log(ethereum);
  if (ethereum) {
    ethereum.on('accountsChanged', function (accounts: string[]) {
      setAddress(accounts[0]);
    });
  }
  const requestAccount = async () => {
    console.log('Connect wallet');
    try {
      const accounts = await ethereum.request({
        method: 'eth_requestAccounts',
      });
      setAddress((accounts as string[])[0]);

      // get the balance from the investor account
      let testTokenBalance = await testToken.methods.balanceOf(address).call();
      console.log(testTokenBalance);
    } catch (error) {
      alert('Metamask not detected!');
    }
  };

  // generate zombie
  const [infomationZombie, setInfomationZombie] = useRecoilState(InfomationZombie);
  const generateZombie = (id: number, name: string, dna: any) => {
    let dnaStr: string = String(dna);
    // pad DNA with leading zeroes if it's less than 16 characters
    while (dnaStr.length < 16) {
      dnaStr = '0' + dnaStr;
    }
    let zombieDetails = {
      // this number:
      headChoice: (parseInt(dnaStr.substring(0, 2)) % 7) + 1,
      // 2nd 2 digits make up the eyes, 11 variations:
      eyeChoice: (parseInt(dnaStr.substring(2, 4)) % 11) + 1,
      // 6 variations of shirts:
      shirtChoice: (parseInt(dnaStr.substring(4, 6)) % 6) + 1,
      // last 6 digits control color. Updated using CSS filter: hue-rotate
      // which has 360 degrees:
      skinColorChoice: (parseInt(dnaStr.substring(6, 8)) / 100) * 360,
      eyeColorChoice: (parseInt(dnaStr.substring(8, 10)) / 100) * 360,
      clothesColorChoice: (parseInt(dnaStr.substring(10, 12)) / 100) * 360,
      zombieName: name,
      zombieDescription: 'A Level 1 CryptoZombie',
    };
    return zombieDetails;
  };
  // create zombie
  const createZombieRandom = async () => {
    await myContract.methods.createRandomZombie(textNameZombie).send({ from: address, gasPrice: 10000000000 });
  };
  // new zombie
  const web3ws = new Web3(new Web3.providers.WebsocketProvider(WS_API));
  const myContractWS = new web3ws.eth.Contract(abi, contractAddress);

  // Listen for the `NewZombie` event, and update the UI
  myContractWS.events.NewZombie(function (error: any, result: any) {
    if (error) return;
    console.log('result: ', result);
    let zombie = result.returnValues;
    console.log('A new zombie was born!', zombie);
    setInfomationZombie(generateZombie(zombie.zombieId, zombie.name, zombie.dna));
  });

  return (
    <div className="App">
      <h1 className="text-3xl  text-center">Project Crypto zombies front end</h1>
      <button className="border-1" onClick={requestAccount}>
        Connect wallet
      </button>
      <div className="relative">
        <Generate />
        <Zombie Style="absolute top-[22%] left-[5%] " gen="135046" />
        <button onClick={createZombieRandom}>Create Zombie</button>
      </div>
    </div>
  );
}
export default App;

declare global {
  interface Window {
    ethereum: {
      removeListener<T>(event: string, cb: (params: T) => void): void;
      request<T>(params: { method: string; params?: any[] }): Promise<T>;
      on<T>(event: string, cb: (params: T) => void): void;
      selectedAddress: string | undefined;
    };
  }
}

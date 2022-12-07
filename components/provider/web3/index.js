import { createContext, useContext, useEffect, useState } from "react";
import {ethers} from "ethers"
import { setupHooks } from "../../../hooks/web3/setupHooks";
import { loadContract } from "./loadContract";

const Web3Context = createContext(null)

const Web3Provider = ({children}) => {

    const [web3Api, setWeb3api] = useState({
        ethereum: null,
        provider: null,
        contract: null,
        isLoading: true,
        hooks: setupHooks({})
    });
    
    useEffect(() => {
        const loadProvider = async () => {
            const ethereum = window.ethereum;
            const provider = new ethers.providers.Web3Provider(ethereum);
            const contract = await loadContract("NftMarket",provider);
            setWeb3api({
                ethereum,
                provider,
                contract,
                isLoading: false,
                hooks: setupHooks({ethereum,provider,contract})
            });
        }
        loadProvider();
    },[])


    return(
        <Web3Context.Provider value={web3Api}>
            {children}
        </Web3Context.Provider>
    )
}

export default Web3Provider

export const useWeb3 = ()=>{
    return useContext(Web3Context)
}

export const useHooks = () => {
    const {hooks} = useWeb3();
    return hooks;
}
import { ethers } from "ethers";

export const loadContract = async (name,provider)=>{
    const res = await fetch(`/contracts/${name}.json`)
    const Artifact = await res.json();

    const contract = new ethers.Contract(
        Artifact.networks[5777].address,
        Artifact.abi,
        provider
    )
    return contract;
}
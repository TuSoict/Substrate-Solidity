import { ethers } from 'ethers';
import useSWR from 'swr'

export const hookFactory = ({ contract }) => () => {
    const { data, ...swr } = useSWR(
        contract ? "web3/useListedNfts" : null,
        async () => {
            const nfts = [];
            const allNfts = await contract.getAllNftsOnSale();

            for(let i = 0; i < allNfts.length; i++){
                const item = allNfts[i];
                const tokenURI = await contract?.tokenURI(item.tokenId);
                const metaRes = await fetch(tokenURI);
                const meta = await metaRes.json();

                nfts.push({
                    price: parseFloat(ethers.utils.formatEther(item.price)),
                    tokenId: item.tokenId.toNumber(),
                    creator: item.creator,
                    isListed: item.isListed,
                    meta
                })
            }

            return nfts;
        }
    )


    return {
        ...swr,
        data,
    }
}
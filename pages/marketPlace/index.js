import MainLayout from "../../components/layout/EmptyLayout"
import { useListedNfts } from "../../hooks/web3"

export default function NftMarketPlace() {
  const {nfts} = useListedNfts();
  console.log(nfts.data);
  return (
    <div>
      <h1 className='text-xl text-red-400'>Nft Market Place</h1> 
    </div>
  )
}

NftMarketPlace.Layout = MainLayout
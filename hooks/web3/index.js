import { useHooks } from "../../components/provider"

export const useAccount = () => {
    const hooks = useHooks();
    const swrRes = hooks.useAccount();

    return{
        account: swrRes
    }
}

export const useListedNfts = () => {
    const hooks = useHooks();
    const swrRes = hooks.useListedNfts();

    return{
        nfts: swrRes
    }
}
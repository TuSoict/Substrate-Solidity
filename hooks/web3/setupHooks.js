import { hookFactory as createUseAccountHook } from "./useAccount";
import { hookFactory as createUseListedHook } from "./useListedNfts";

export const setupHooks = (deps)=>{
    return {
        useAccount: createUseAccountHook(deps),
        useListedNfts: createUseListedHook(deps)
    }
}
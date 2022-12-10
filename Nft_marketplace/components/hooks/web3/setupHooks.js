import { hookFactory as createAccountHook, useAccountHook } from "./useAccount";
import { hookFactory as createNetworkHook, useNetworkHook } from "./useNetwork";

export const setupHooks = (deps) => {
  return {
    useAccount: createAccountHook(deps),
    useNetwork: createNetworkHook(deps),
  };
};

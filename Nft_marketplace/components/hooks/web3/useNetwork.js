import useSWR from "swr";

const NETWORKS = {
  1: "Ethereum Mainet",
  3: "Ropsten Test Net",
  4: "Rinkeby Test Net",
  5: "Goerli Test Net",
  42: "Kovan Test Net",
  56: "Binance SmartChain",
  97: "Binance SmartChain Testnet",
  1337: "Ganache",
};

export const hookFactory =
  ({ provider, isLoading }) =>
  () => {
    const { data, mutate, isValidating, ...swr } = useSWR(
      provider ? "web3/useNetwork" : null,
      async () => {
        const network = await provider?.getNetwork();
        const chainId = await network?.chainId;

        return NETWORKS[chainId];
      },
      {
        revalidateOnFocus: false,
      }
    );
    return {
      ...swr,
      mutate,
      data,
      isLoading: isLoading || isValidating,
    };
  };

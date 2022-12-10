import useSWR from "swr";
import { useEffect } from "react";

export const hookFactory =
  ({ provider, ethereum, isLoading }) =>
  () => {
    const { data, mutate, isValidating, ...swr } = useSWR(
      provider ? "web3/account" : null,
      async () => {
        const accounts = await provider?.listAccounts();
        const account = accounts[0];
        return account;
      },
      {
        revalidateOnFocus: false,
      }
    );

    useEffect(() => {
      ethereum?.on("accountsChanged", handleAccountsChanged);
      return () => {
        ethereum?.removeListener("accountsChanged", handleAccountsChanged);
      };
    });

    const handleAccountsChanged = async (...args) => {
      const accounts = args[0];
      if (accounts.length === 0) {
        throw "Please! Connect to Wallet";
      } else if (accounts[0] !== data) {
        mutate(accounts[0]);
      }
    };

    const connect = async () => {
      try {
        ethereum?.request({ method: "eth_requestAccounts" });
      } catch (err) {
        console.log(err);
      }
    };

    return {
      ...swr,
      mutate,
      data,
      isLoading: isLoading || isValidating,
      isInstalled: ethereum?.isMetaMask || false,
      connect,
    };
  };

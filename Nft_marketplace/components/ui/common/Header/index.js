import { useAccount, useNetwork } from "../../../hooks/web3/index";

export default function Header() {
  const { account } = useAccount();
  const { network } = useNetwork();
  console.log(network.data);

  return (
    <>
      {" "}
      <div className="flex gap-[17px]">
        <div className="bg-red-800 px-2 py-0.5 rounded-md text-white">
          {network.data}
        </div>
        {account.isLoading ? (
          <button
            type="button"
            className="rounded-full bg-indigo-700 px-2 py-0.5 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
          >
            is Loading
          </button>
        ) : !account.isInstalled ? (
          <button
            type="button"
            className="rounded-full bg-indigo-700 px-2 py-0.5 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
            onClick={() => {
              window.open("https://metamask.io/download/", "_blank");
            }}
          >
            Install MetaMask
          </button>
        ) : account.data ? (
          <button
            type="button"
            className="rounded-full bg-indigo-700 px-2 py-0.5 text-white hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
          >
            {account.data.substring(0, 4)}...
            {account.data.substring(account.data.length - 4)}
          </button>
        ) : (
          <button
            type="button"
            className="rounded-full bg-indigo-700 px-2 py-0.5 text-gray-400 hover:text-white focus:outline-none focus:ring-2 focus:ring-white focus:ring-offset-2 focus:ring-offset-gray-800"
            onClick={account.connect}
          >
            Connect Wallet
          </button>
        )}
      </div>
    </>
  );
}

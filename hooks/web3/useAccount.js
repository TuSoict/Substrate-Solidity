import useSWR from 'swr'

export const hookFactory = ({provider, ethereum, isLoading}) => () => {
    const {data, error, mutate, isValidating, ...swr} = useSWR(
        provider ? "web3/useAccount" : null,
        async () => {
            const accounts = await provider?.listAccounts();
            const account = accounts[0]
            return account
        }
    )

    const connect = async () =>{
        try{
            await ethereum.request({method:"eth_requestAccounts"});
        }
        catch(error){
            console.log(error);
        }
    }

    return {
        ...swr,
        data,
        mutate,
        error,
        isValidating,
        connect
    }
}
import { StdFee } from '@cosmjs/stargate';
import { useChain } from '@cosmos-kit/react';
import {ExecuteMsg,QueryMsg,ConstellationQueryClient} from "../config/constellation"
import async from 'react-select/dist/declarations/src/async/index';


export const useQueryClient =async (chainName: string,contractAddress:string) => {
  const { address, getCosmWasmClient } =
    useChain(chainName);

    if (!address) {
      return;
    }

    let client: Awaited<ReturnType<typeof getCosmWasmClient>>;
    let QueryClient: ConstellationQueryClient;
    try {
      client = await getCosmWasmClient();
      QueryClient = new ConstellationQueryClient(client,contractAddress)
    } catch (e: any) {
      console.error(e);
      return;
    }
  return { QueryClient };
}
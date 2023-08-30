import { StdFee } from '@cosmjs/stargate';
import { useToaster, ToastType, type CustomToast } from './useToaster';
import { useChain } from '@cosmos-kit/react';
import { ToastId } from '@chakra-ui/react';
import {ExecuteMsg,ConstellationClient} from "../config/constellation"
import { Coin } from '@cosmjs/amino';

interface Msg {
  typeUrl: string;
  value: any;
}

interface TxOptions {
  fee?: StdFee | null;
  toast?: Partial<CustomToast>;
  onSuccess?: () => void;
}

export enum TxStatus {
  Failed = 'Transaction Failed',
  Successful = 'Transaction Successful',
  Broadcasting = 'Transaction Broadcasting',
}

export const useTx = (chainName: string,denom: string,contractAddress:string) => {
  const { address, getSigningCosmWasmClient } =
    useChain(chainName);

  const toaster = useToaster();
  const AddString = (params:string[]) => {
    let ans = "";
    for (let param in params){
        ans = ans+param;
    }
    return ans;
  }
  const tx = async (msgs: ExecuteMsg,options: TxOptions, _funds?: Coin[]) => {
    if (!address) {
      toaster.toast({
        type: ToastType.Error,
        title: 'Wallet not connected',
        message: 'Please connect the wallet',
      });
      return;
    }

    let client: Awaited<ReturnType<typeof getSigningCosmWasmClient>>;
    let wasmClient: Awaited<ConstellationClient>;
    let fee: StdFee;
    try {
      fee = {
        amount:[{denom:denom,amount:"125000"}],
        gas:"1000000"
      };
      client = await getSigningCosmWasmClient();
      wasmClient = new ConstellationClient(client,address,contractAddress)
    } catch (e: any) {
      console.error(e);
      toaster.toast({
        title: TxStatus.Failed,
        message: e?.message || 'An unexpected error has occured',
        type: ToastType.Error,
      });
      return;
    }

    let broadcastToastId: ToastId;

    broadcastToastId = toaster.toast({
      title: TxStatus.Broadcasting,
      message: 'Waiting for transaction to be included in the block',
      type: ToastType.Loading,
      duration: 999999,
    });

    if (wasmClient) {
      await wasmClient.client
      .execute(wasmClient.sender, wasmClient.contractAddress, msgs, fee, "", _funds)
        .then((res) => {
          if (res.height) {
            if (options.onSuccess) options.onSuccess();

            toaster.toast({
              title: options.toast?.title || TxStatus.Successful,
              type: options.toast?.type || ToastType.Success,
              message: res.transactionHash,
            });
          } else {
            toaster.toast({
              title: TxStatus.Failed,
              message: AddString(res?.logs.map((log)=>{
                return log.log
              })),
              type: ToastType.Error,
              duration: 10000,
            });
          }
        })
        .catch((err) => {
          toaster.toast({
            title: TxStatus.Failed,
            message: err?.message,
            type: ToastType.Error,
            duration: 10000,
          });
        })
        .finally(() => toaster.close(broadcastToastId));
    } else {
      toaster.close(broadcastToastId);
    }
  };

  return { tx };
};

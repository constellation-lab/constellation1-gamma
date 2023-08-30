import '../styles/globals.css';
import type { AppProps } from 'next/app';
import { ChainProvider } from '@cosmos-kit/react';
import { ChakraProvider } from '@chakra-ui/react';
import { wallets as keplrWallets } from '@cosmos-kit/keplr';

import { SignerOptions } from '@cosmos-kit/core';
import { Chain, AssetList } from '@chain-registry/types';

import { defaultTheme } from '../config';
import {nibiruChian,nibiruAssets} from '../config/nibiru_itn_1';
import '@interchain-ui/react/styles';

function CreateCosmosApp({ Component, pageProps }: AppProps) {
  const signerOptions: SignerOptions = {
    // signingStargate: () => {
    //   return getSigningCosmosClientOptions();
    // }
  };
  

  return (
    <ChakraProvider theme={defaultTheme}>
      <ChainProvider
        chains={[nibiruChian]}
        assetLists={[nibiruAssets]}
        wallets={[...keplrWallets]}
        walletConnectOptions={{
          signClient: {
            projectId: 'a8510432ebb71e6948cfd6cde54b70f7',
            relayUrl: 'wss://relay.walletconnect.org',
            metadata: {
              name: 'CosmosKit Template',
              description: 'CosmosKit dapp template',
              url: 'https://docs.cosmoskit.com/',
              icons: [],
            },
          },
        }}
        signerOptions={signerOptions}
      >
        <Component {...pageProps} />
      </ChainProvider>
    </ChakraProvider>
  );
}

export default CreateCosmosApp;

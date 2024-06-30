import React, { useState } from 'react';
import {
  Box,
  Button,
  Flex,
  FormControl,
  FormLabel,
  Input,
  Select,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { LiquidityPoolClient } from '../config/liquidity_pool/liquidity_pool.client';
import { LIQUIDITY_POOL_CONTRACT } from '../config/defaults';
import { Asset } from '../config/liquidity_pool/liquidity_pool.types';
import { chainName } from '../config';

const LiquidityProvisionView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [tokenPair, setTokenPair] = useState<string>('');
  const [amount, setAmount] = useState<string>('');

  const handleAddLiquidity = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    const [token1, token2] = tokenPair.split('-');
    try {
      const client = await getSigningCosmWasmClient();
      const liquidityPoolClient = new LiquidityPoolClient(client, address, LIQUIDITY_POOL_CONTRACT);
      const assets: Asset[] = [
        {
          denom: token1,
          amount: amount,
        },
        {
          denom: token2,
          amount: amount,
        },
      ];
      const response = await liquidityPoolClient.deposit({ assets });
      console.log('Add Liquidity response:', response);
      alert('Liquidity added successfully!');
    } catch (error) {
      console.error('Add Liquidity error:', error);
      alert('Adding liquidity failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="tokenPair" mb={4}>
        <FormLabel>Token Pair</FormLabel>
        <Select
          placeholder="Select token pair"
          value={tokenPair}
          onChange={(e) => setTokenPair(e.target.value)}
        >
          {/* Render token pair options */}
        </Select>
      </FormControl>
      <FormControl id="amount" mb={4}>
        <FormLabel>Amount of Tokens to Add</FormLabel>
        <Input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleAddLiquidity} isDisabled={!isWalletConnected}>
          Add Liquidity
        </Button>
      </Flex>
    </Box>
  );
};

export default LiquidityProvisionView;
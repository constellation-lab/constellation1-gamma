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
import { chainName, LIQUIDITY_POOL_CONTRACT } from '../config';
import { LiquidityPoolClient } from '../config/liquidity_pool/liquidity_pool.client';
import { Asset } from '../config/liquidity_pool/liquidity_pool.types';

const SwapView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [fromToken, setFromToken] = useState('');
  const [toToken, setToToken] = useState('');
  const [amount, setAmount] = useState('');

  const handleSwap = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }

    try {
      const client = await getSigningCosmWasmClient();
      const liquidityPoolClient = new LiquidityPoolClient(client, address, LIQUIDITY_POOL_CONTRACT);

      const assets: Asset[] = [
        {
          denom: fromToken,
          amount: amount,
        },
        {
          denom: toToken,
          amount: '0', // The amount to receive will be calculated by the contract
        },
      ];

      const response = await liquidityPoolClient.provideLiquidity(
        { assets },
        'auto',
        '',
        [{ denom: fromToken, amount: amount }]
      );

      console.log('Swap response:', response);
      alert('Swap successful!');
    } catch (error) {
      console.error('Swap error:', error);
      alert('Swap failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="fromToken" mb={4}>
        <FormLabel>From Token</FormLabel>
        <Select
          placeholder="Select token"
          value={fromToken}
          onChange={(e) => setFromToken(e.target.value)}
        >
          {/* Render token options */}
        </Select>
      </FormControl>
      <FormControl id="toToken" mb={4}>
        <FormLabel>To Token</FormLabel>
        <Select
          placeholder="Select token"
          value={toToken}
          onChange={(e) => setToToken(e.target.value)}
        >
          {/* Render token options */}
        </Select>
      </FormControl>
      <FormControl id="amount" mb={4}>
        <FormLabel>Amount to Swap</FormLabel>
        <Input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleSwap} isDisabled={!isWalletConnected}>
          Swap
        </Button>
      </Flex>
    </Box>
  );
};

export default SwapView;
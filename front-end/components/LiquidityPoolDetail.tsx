import React, { useEffect, useState } from 'react';
import { Box, Heading, Text, Button, Flex, useColorMode, useColorModeValue } from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { LiquidityPool, Asset } from '../config/liquidity_pool/liquidity_pool.types';
import { LiquidityPoolClient, LiquidityPoolQueryClient } from '../config/liquidity_pool/liquidity_pool.client';
import { LIQUIDITY_POOL_CONTRACT } from '../config/defaults';
import { chainName } from '../config';

const LiquidityPoolDetail: React.FC<{ poolId: string }> = ({ poolId }) => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getCosmWasmClient, getSigningCosmWasmClient, address } = useChain(chainName);
  const [pool, setPool] = useState<LiquidityPool | null>(null);
  const [amount, setAmount] = useState<string>('');

  useEffect(() => {
    const fetchLiquidityPool = async () => {
      if (isWalletConnected && address) {
        try {
          const client = await getCosmWasmClient();
          const liquidityPoolQueryClient = new LiquidityPoolQueryClient(client, LIQUIDITY_POOL_CONTRACT);
          const result = await liquidityPoolQueryClient.getPool();
          setPool(result);
        } catch (error) {
          console.error('Error fetching liquidity pool:', error);
        }
      }
    };
    fetchLiquidityPool();
  }, [isWalletConnected, address, getCosmWasmClient, poolId]);

  const handleAddLiquidity = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    try {
      const client = await getSigningCosmWasmClient();
      const liquidityPoolClient = new LiquidityPoolClient(client, address, LIQUIDITY_POOL_CONTRACT);
      const assets: Asset[] = [
        {
          denom: pool!.assets[0].denom,
          amount: amount,
        },
        {
          denom: pool!.assets[1].denom,
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

  if (!pool) {
    return <Text>Loading...</Text>;
  }

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Heading size="xl" mb={4}>
        Liquidity Pool: {pool.assets[0].denom}/{pool.assets[1].denom}
      </Heading>
      <Text mb={4}>Total Liquidity: {pool.lp_token_supply}</Text>
      <Text mb={4}>Token 1: {pool.assets[0].denom}</Text>
      <Text mb={4}>Token 2: {pool.assets[1].denom}</Text>
      <Flex align="center" mb={4}>
        <Text mr={2}>Amount:</Text>
        <input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      </Flex>
      <Button onClick={handleAddLiquidity} colorScheme="blue">
        Add Liquidity
      </Button>
    </Box>
  );
};

export default LiquidityPoolDetail;
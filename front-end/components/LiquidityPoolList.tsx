import React, { useEffect, useState } from 'react';
import { Box, Heading, Text, Button, Flex, useColorMode, useColorModeValue } from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { LIQUIDITY_POOL_CONTRACT } from '../config/defaults';
import { LiquidityPoolQueryClient } from '../config/liquidity_pool/liquidity_pool.client';
import { LiquidityPool } from '../config/liquidity_pool/liquidity_pool.types';
import { chainName } from '../config';

const LiquidityPoolList: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getCosmWasmClient, address } = useChain(chainName);
  const [pools, setPools] = useState<LiquidityPool[]>([]);

  useEffect(() => {
    const fetchLiquidityPools = async () => {
      if (isWalletConnected && address) {
        try {
          const client = await getCosmWasmClient();
          const liquidityPoolQueryClient = new LiquidityPoolQueryClient(client, LIQUIDITY_POOL_CONTRACT);
          const result = await liquidityPoolQueryClient.getPool();
          setPools([result]);
        } catch (error) {
          console.error('Error fetching liquidity pools:', error);
        }
      }
    };
    fetchLiquidityPools();
  }, [isWalletConnected, address, getCosmWasmClient]);

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Heading size="xl" mb={4}>
        Liquidity Pools
      </Heading>
      {pools.map((pool) => (
        <Box key={pool.lp_token_supply} p={4} mb={4} borderWidth={1} borderRadius="md">
          <Flex justify="space-between" align="center">
            <Text fontWeight="bold">{pool.assets[0].denom}/{pool.assets[1].denom}</Text>
            <Button size="sm" colorScheme="blue">View Details</Button>
          </Flex>
        </Box>
      ))}
    </Box>
  );
};

export default LiquidityPoolList;
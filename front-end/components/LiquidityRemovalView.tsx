import React, { useState } from 'react';
import {
  Box,
  Button,
  Flex,
  FormControl,
  FormLabel,
  Input,
  Slider,
  SliderTrack,
  SliderFilledTrack,
  SliderThumb,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { LiquidityPoolClient } from '../config/liquidity_pool/liquidity_pool.client';
import { LIQUIDITY_POOL_CONTRACT } from '../config/defaults';
import { chainName } from '../config';

const LiquidityRemovalView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [tokenPair, setTokenPair] = useState<string>('');
  const [amount, setAmount] = useState<number>(0);

  const handleRemoveLiquidity = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    try {
      const client = await getSigningCosmWasmClient();
      const liquidityPoolClient = new LiquidityPoolClient(client, address, LIQUIDITY_POOL_CONTRACT);
      const response = await liquidityPoolClient.withdraw({ lpTokens: amount.toString() });
      console.log('Remove Liquidity response:', response);
      alert('Liquidity removed successfully!');
    } catch (error) {
      console.error('Remove Liquidity error:', error);
      alert('Removing liquidity failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="tokenPair" mb={4}>
        <FormLabel>Token Pair</FormLabel>
        <Input
          type="text"
          value={tokenPair}
          onChange={(e) => setTokenPair(e.target.value)}
        />
      </FormControl>
      <FormControl id="amount" mb={4}>
        <FormLabel>Amount of Liquidity to Remove</FormLabel>
        <Slider
          aria-label="Amount"
          value={amount}
          onChange={(value) => setAmount(value)}
          min={0}
          max={100}
        >
          <SliderTrack>
            <SliderFilledTrack />
          </SliderTrack>
          <SliderThumb />
        </Slider>
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleRemoveLiquidity} isDisabled={!isWalletConnected}>
          Remove Liquidity
        </Button>
      </Flex>
    </Box>
  );
};

export default LiquidityRemovalView;
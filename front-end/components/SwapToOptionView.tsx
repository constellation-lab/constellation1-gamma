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
import { coins } from '@cosmjs/proto-signing';
import { chainName, LIQUIDITY_POOL_CONTRACT } from '../config';
import { LiquidityPoolClient } from '../config/liquidity_pool/liquidity_pool.client';
import { Asset } from '../config/liquidity_pool/liquidity_pool.types';

const SwapToOptionView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [fromToken, setFromToken] = useState('');
  const [strikePrice, setStrikePrice] = useState('');
  const [expirationDate, setExpirationDate] = useState('');
  const [amount, setAmount] = useState('');

  const handleSwapToOption = async () => {
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
      ];

      const response = await liquidityPoolClient.deposit(
        { assets },
        'auto',
        '',
        coins(amount, fromToken)
      );

      console.log('Swap to Option response:', response);
      alert('Swap to Option successful!');
    } catch (error) {
      console.error('Swap to Option error:', error);
      alert('Swap to Option failed!');
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
      <FormControl id="strikePrice" mb={4}>
        <FormLabel>Strike Price</FormLabel>
        <Input
          type="number"
          value={strikePrice}
          onChange={(e) => setStrikePrice(e.target.value)}
        />
      </FormControl>
      <FormControl id="expirationDate" mb={4}>
        <FormLabel>Expiration Date</FormLabel>
        <Input
          type="date"
          value={expirationDate}
          onChange={(e) => setExpirationDate(e.target.value)}
        />
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
        <Button onClick={handleSwapToOption} isDisabled={!isWalletConnected}>
          Swap to Option
        </Button>
      </Flex>
    </Box>
  );
};

export default SwapToOptionView;

/*If your liquidity pool contract has a different method for swapping 
tokens, you should use that method instead. The exact implementation will 
depend on your specific smart contract.*/

/* const handleSwapToOption = async () => {
  if (!isWalletConnected || !address) {
    alert('Please connect your wallet first!');
    return;
  }

  try {
    const client = await getSigningCosmWasmClient();
    const liquidityPoolClient = new LiquidityPoolClient(client, address, LIQUIDITY_POOL_CONTRACT);

    // Assuming your contract has a swapToOption method
    const response = await liquidityPoolClient.swapToOption({
      from_token: fromToken,
      amount: amount,
      strike_price: strikePrice,
      expiration_date: expirationDate
    });

    console.log('Swap to Option response:', response);
    alert('Swap to Option successful!');
  } catch (error) {
    console.error('Swap to Option error:', error);
    alert('Swap to Option failed!');
  }
};*/
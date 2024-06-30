import React, { useState } from 'react';
import {
  Box,
  Button,
  Flex,
  FormControl,
  FormLabel,
  Input,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { chainName, INCENTIVE_MANAGER_CONTRACT } from '../config';
import { IncentiveManagerClient } from '../config/incentive_manager/incentive_manager.client';

const UnstakingView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [amount, setAmount] = useState('');

  const handleUnstake = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }

    try {
      const client = await getSigningCosmWasmClient();
      const incentiveManagerClient = new IncentiveManagerClient(client, address, INCENTIVE_MANAGER_CONTRACT);

      const response = await incentiveManagerClient.unstake({
        amount,
        program_id: 'default', // Adjust this if you have multiple staking programs
      });

      console.log('Unstake response:', response);
      alert('Unstaking successful!');
    } catch (error) {
      console.error('Unstake error:', error);
      alert('Unstaking failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="amount" mb={4}>
        <FormLabel>Amount to Unstake</FormLabel>
        <Input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleUnstake} isDisabled={!isWalletConnected}>
          Unstake
        </Button>
      </Flex>
    </Box>
  );
};

export default UnstakingView;
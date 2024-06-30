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
import { coins } from '@cosmjs/proto-signing';

const StakingView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [amount, setAmount] = useState('');

  const handleStake = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }

    try {
      const client = await getSigningCosmWasmClient();
      const incentiveManagerClient = new IncentiveManagerClient(client, address, INCENTIVE_MANAGER_CONTRACT);

      const response = await incentiveManagerClient.stake(
        {
          amount,
          program_id: 'default', // Adjust this if you have multiple staking programs
        },
        'auto',
        '',
        coins(amount, 'unibi')
      );

      console.log('Stake response:', response);
      alert('Staking successful!');
    } catch (error) {
      console.error('Stake error:', error);
      alert('Staking failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="amount" mb={4}>
        <FormLabel>Amount to Stake</FormLabel>
        <Input
          type="number"
          value={amount}
          onChange={(e) => setAmount(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleStake} isDisabled={!isWalletConnected}>
          Stake
        </Button>
      </Flex>
    </Box>
  );
};

export default StakingView;
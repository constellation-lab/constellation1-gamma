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
import { chainName, INCENTIVE_MANAGER_CONTRACT } from '../config/defaults';
import { IncentiveManagerClient } from '../config/incentive_manager';

const IncentiveManagementView = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { getSigningCosmWasmClient, address } = useChain(chainName);
  const [programId, setProgramId] = useState('');
  const [rewardToken, setRewardToken] = useState('');
  const [rewardAmount, setRewardAmount] = useState('');
  const [duration, setDuration] = useState('');

  const handleCreateIncentiveProgram = async () => {
    if (!address) {
      alert('Please connect your wallet first!');
      return;
    }

    const client = await getSigningCosmWasmClient();
    const incentiveManagerClient = new IncentiveManagerClient(client, address, INCENTIVE_MANAGER_CONTRACT);

    try {
      const response = await incentiveManagerClient.createYieldFarmingProgram({
        program_id: programId,
        reward_token: rewardToken,
        reward_rate: rewardAmount,
        start_time: Math.floor(Date.now() / 1000),
        end_time: Math.floor(Date.now() / 1000) + Number(duration) * 24 * 60 * 60,
      });
      console.log('Create Incentive Program response:', response);
      alert('Incentive program created successfully!');
    } catch (error) {
      console.error('Create Incentive Program error:', error);
      alert('Creating incentive program failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="programId" mb={4}>
        <FormLabel>Program ID</FormLabel>
        <Input
          type="text"
          value={programId}
          onChange={(e) => setProgramId(e.target.value)}
        />
      </FormControl>
      <FormControl id="rewardToken" mb={4}>
        <FormLabel>Reward Token</FormLabel>
        <Input
          type="text"
          value={rewardToken}
          onChange={(e) => setRewardToken(e.target.value)}
        />
      </FormControl>
      <FormControl id="rewardAmount" mb={4}>
        <FormLabel>Reward Amount</FormLabel>
        <Input
          type="number"
          value={rewardAmount}
          onChange={(e) => setRewardAmount(e.target.value)}
        />
      </FormControl>
      <FormControl id="duration" mb={4}>
        <FormLabel>Duration (in days)</FormLabel>
        <Input
          type="number"
          value={duration}
          onChange={(e) => setDuration(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleCreateIncentiveProgram}>Create Incentive Program</Button>
      </Flex>
    </Box>
  );
};

export default IncentiveManagementView;
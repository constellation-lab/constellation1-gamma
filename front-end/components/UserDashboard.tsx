import React, { useEffect, useState } from 'react';
import {
  Box,
  Button,
  Flex,
  Text,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { chainName, INCENTIVE_MANAGER_CONTRACT } from '../config';
import { IncentiveManagerQueryClient } from '../config/incentive_manager/incentive_manager.client';

interface UserInfo {
  account_balance: string;
  transaction_history: string[];
  positions: string[];
  staking_details: {
    staked_amount: string;
    reward_earned: string;
  };
}

const UserDashboard: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getCosmWasmClient, address } = useChain(chainName);
  const [userInfo, setUserInfo] = useState<UserInfo | null>(null);

  useEffect(() => {
    const fetchUserInfo = async () => {
      if (isWalletConnected && address) {
        try {
          const client = await getCosmWasmClient();
          const incentiveManagerQueryClient = new IncentiveManagerQueryClient(client, INCENTIVE_MANAGER_CONTRACT);

          // Assuming there's a getUserInfo method in the IncentiveManagerQueryClient
          const result = await incentiveManagerQueryClient.getUserInfo(address);
          setUserInfo(result);
        } catch (error) {
          console.error('Error fetching user info:', error);
        }
      }
    };
    fetchUserInfo();
  }, [isWalletConnected, address, getCosmWasmClient]);

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Text fontSize="xl" fontWeight="bold" mb={4}>
        User Dashboard
      </Text>
      {userInfo ? (
        <>
          <Text mb={2}>Account Balance: {userInfo.account_balance}</Text>
          <Text mb={2}>Transaction History:</Text>
          <Box>
            {userInfo.transaction_history.map((transaction, index) => (
              <Text key={index} mb={1}>
                {transaction}
              </Text>
            ))}
          </Box>
          <Text mb={2}>Positions:</Text>
          <Box>
            {userInfo.positions.map((position, index) => (
              <Text key={index} mb={1}>
                {position}
              </Text>
            ))}
          </Box>
          <Text mb={2}>Staking Details:</Text>
          <Box>
            <Text mb={1}>Staked Amount: {userInfo.staking_details.staked_amount}</Text>
            <Text mb={1}>Reward Earned: {userInfo.staking_details.reward_earned}</Text>
          </Box>
        </>
      ) : (
        <Text>Loading user info...</Text>
      )}
      <Flex mt={4}>
        <Button colorScheme="blue">Claim Rewards</Button>
      </Flex>
    </Box>
  );
};

export default UserDashboard;
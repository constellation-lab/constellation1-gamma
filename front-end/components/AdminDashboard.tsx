import React, { useEffect, useState } from 'react';
import {
  Box,
  Button,
  Flex,
  Text,
  useColorMode,
  useColorModeValue,
  BoxProps,
  TextProps,
  ButtonProps,
  FlexProps,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { chainName } from '../config';
import { IncentiveManagerQueryClient, PoolInfo } from '../config/incentive_manager';
import { INCENTIVE_MANAGER_CONTRACT } from '../config/defaults';

const AdminDashboard: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { getCosmWasmClient, address } = useChain(chainName);
  const [statistics, setStatistics] = useState<PoolInfo | null>(null);

  useEffect(() => {
    const fetchStatistics = async () => {
      if (address) {
        const client = await getCosmWasmClient();
        const incentiveManagerQueryClient = new IncentiveManagerQueryClient(client, INCENTIVE_MANAGER_CONTRACT);
        try {
          const poolInfo = await incentiveManagerQueryClient.getPoolInfo();
          setStatistics(poolInfo);
        } catch (error) {
          console.error('Error fetching system statistics:', error);
        }
      }
    };
    fetchStatistics();
  }, [address, getCosmWasmClient]);

  const handleManageParameters = () => {
    // Navigate to the parameter management view
  };

  const handleManageIncentives = () => {
    // Navigate to the incentive management view
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Text fontSize="xl" fontWeight="bold" mb={4} as="h1">
        Admin Dashboard
      </Text>
      {statistics ? (
        <>
          <Text mb={2}>Total Liquidity: {statistics.total_liquidity}</Text>
          {/* Display other statistics */}
        </>
      ) : (
        <Text>Loading system statistics...</Text>
      )}
      <Flex mt={4}>
        <Button colorScheme="blue" mr={4} onClick={handleManageParameters}>
          Manage Parameters
        </Button>
        <Button colorScheme="blue" onClick={handleManageIncentives}>
          Manage Incentives
        </Button>
      </Flex>
    </Box>
  );
};

export default AdminDashboard;
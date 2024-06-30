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
import { chainName, RISK_MANAGEMENT_CONTRACT } from '../config';
import { RiskManagementClient } from '../config/risk_management/risk_management.client';
import { Decimal } from '../config/risk_management/risk_management.types';

const ParameterManagementView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [circuitBreaker, setCircuitBreaker] = useState(false);
  const [positionLimit, setPositionLimit] = useState('');

  const handleUpdateParameters = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    try {
      const client = await getSigningCosmWasmClient();
      const riskManagementClient = new RiskManagementClient(client, address, RISK_MANAGEMENT_CONTRACT);
      const response = await riskManagementClient.adjustParameters({
        volatility_multiplier: positionLimit as Decimal,
      });
      console.log('Update Parameters response:', response);
      alert('Parameters updated successfully!');
    } catch (error) {
      console.error('Update Parameters error:', error);
      alert('Updating parameters failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="circuitBreaker" mb={4}>
        <FormLabel>Circuit Breaker</FormLabel>
        <Input
          type="checkbox"
          checked={circuitBreaker}
          onChange={(e) => setCircuitBreaker(e.target.checked)}
        />
      </FormControl>
      <FormControl id="positionLimit" mb={4}>
        <FormLabel>Position Limit</FormLabel>
        <Input
          type="number"
          value={positionLimit}
          onChange={(e) => setPositionLimit(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleUpdateParameters} isDisabled={!isWalletConnected}>
          Update Parameters
        </Button>
      </Flex>
    </Box>
  );
};

export default ParameterManagementView;
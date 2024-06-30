import React from 'react';
import {
  Box,
  Button,
  Flex,
  Text,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';

interface TransactionConfirmationViewProps {
  transaction: any;
  onConfirm: () => void;
  onCancel: () => void;
}

const TransactionConfirmationView: React.FC<TransactionConfirmationViewProps> = ({ 
  transaction, 
  onConfirm, 
  onCancel 
}) => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Text fontSize="xl" fontWeight="bold" mb={4}>
        Confirm Transaction
      </Text>
      <Box mb={4}>
        <Text mb={2}>Transaction Details:</Text>
        <Text>{JSON.stringify(transaction, null, 2)}</Text>
      </Box>
      <Text mb={2}>Gas Fee: 0.025 UNIBI</Text>
      <Flex justify="space-around">
        <Button colorScheme="green" onClick={onConfirm}>
          Confirm
        </Button>
        <Button colorScheme="red" onClick={onCancel}>
          Cancel
        </Button>
      </Flex>
    </Box>
  );
};

export default TransactionConfirmationView;
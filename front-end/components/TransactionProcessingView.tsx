import React from 'react';
import {
  Box,
  Button,
  Flex,
  Text,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';

interface TransactionProcessingViewProps {
  transaction: any;
  status: string;
  hash: string;
  onClose: () => void;
}

const TransactionProcessingView: React.FC<TransactionProcessingViewProps> = ({ 
  transaction, 
  status, 
  hash, 
  onClose 
}) => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Text fontSize="xl" fontWeight="bold" mb={4}>
        Transaction Processing
      </Text>
      <Text mb={2}>Transaction Status: {status}</Text>
      <Text mb={4}>Transaction Hash: {hash}</Text>
      <Flex justify="center">
        <Button onClick={onClose}>Close</Button>
      </Flex>
    </Box>
  );
};

export default TransactionProcessingView;
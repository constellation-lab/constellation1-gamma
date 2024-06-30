import React, { useEffect, useState } from 'react';
import {
  Box,
  Button,
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { chainName, CDT_TOKEN_CONTRACT } from '../config';
import { CdtTokenQueryClient } from '../config/cdt_token/cdt_token.client';
import { Proposal } from '../config/cdt_token/cdt_token.types';

const ProposalListView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getCosmWasmClient, address } = useChain(chainName);
  const [proposals, setProposals] = useState<Proposal[]>([]);

  useEffect(() => {
    const fetchProposals = async () => {
      if (isWalletConnected && address) {
        try {
          const client = await getCosmWasmClient();
          const cdtTokenQueryClient = new CdtTokenQueryClient(client, CDT_TOKEN_CONTRACT);
          const result = await cdtTokenQueryClient.listProposals();
          setProposals(result.proposals);
        } catch (error) {
          console.error('Error fetching proposals:', error);
        }
      }
    };
    fetchProposals();
  }, [isWalletConnected, address, getCosmWasmClient]);

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>ID</Th>
            <Th>Title</Th>
            <Th>Description</Th>
            <Th>Status</Th>
            <Th>Actions</Th>
          </Tr>
        </Thead>
        <Tbody>
          {proposals.map((proposal) => (
            <Tr key={proposal.id}>
              <Td>{proposal.id}</Td>
              <Td>{proposal.title}</Td>
              <Td>{proposal.description}</Td>
              <Td>{proposal.status}</Td>
              <Td>
                <Button size="sm" colorScheme="blue">
                  View Details
                </Button>
              </Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </Box>
  );
};

export default ProposalListView;
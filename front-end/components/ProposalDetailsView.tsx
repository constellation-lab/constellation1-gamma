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
import { chainName, CDT_TOKEN_CONTRACT } from '../config';
import { CdtTokenClient, CdtTokenQueryClient } from '../config/cdt_token/cdt_token.client';
import { Proposal, VotingPowerResponse } from '../config/cdt_token/cdt_token.types';

interface ProposalDetailsViewProps {
  proposalId: number;
} 

const ProposalDetailsView: React.FC<ProposalDetailsViewProps> = ({ proposalId }) => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getCosmWasmClient, getSigningCosmWasmClient, address } = useChain(chainName);
  const [proposal, setProposal] = useState<Proposal | null>(null);
  const [votingPower, setVotingPower] = useState<string>('0');

  useEffect(() => {
    const fetchProposal = async () => {
      if (isWalletConnected && address) {
        try {
          const client = await getCosmWasmClient();
          const cdtTokenQueryClient = new CdtTokenQueryClient(client, CDT_TOKEN_CONTRACT);
          const result = await cdtTokenQueryClient.getProposal(proposalId);
          setProposal(result);
          const votingPowerResult = await cdtTokenQueryClient.votingPower(address);
          setVotingPower(votingPowerResult.voting_power);
        } catch (error) {
          console.error('Error fetching proposal:', error);
        }
      }
    };
    fetchProposal();
  }, [isWalletConnected, address, getCosmWasmClient, proposalId]);

  const handleVote = async (vote: boolean) => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    try {
      const client = await getSigningCosmWasmClient();
      const cdtTokenClient = new CdtTokenClient(client, address, CDT_TOKEN_CONTRACT);
      const response = await cdtTokenClient.vote({
        proposalId,
        vote,
      });
      console.log('Vote response:', response);
      alert('Vote submitted successfully!');
    } catch (error) {
      console.error('Vote error:', error);
      alert('Voting failed!');
    }
  };

  if (!proposal) {
    return <Text>Loading...</Text>;
  }

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <Text fontSize="xl" fontWeight="bold" mb={4}>
        {proposal.title}
      </Text>
      <Text mb={4}>{proposal.description}</Text>
      <Text mb={4}>Status: {proposal.status}</Text>
      <Text mb={4}>Voting Power: {votingPower}</Text>
      <Flex justify="space-around">
        <Button colorScheme="green" onClick={() => handleVote(true)} isDisabled={!isWalletConnected}>
          Vote Yes
        </Button>
        <Button colorScheme="red" onClick={() => handleVote(false)} isDisabled={!isWalletConnected}>
          Vote No
        </Button>
      </Flex>
    </Box>
  );
};

export default ProposalDetailsView;
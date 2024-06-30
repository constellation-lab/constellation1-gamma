import React, { useState } from 'react';
import {
  Box,
  Button,
  Flex,
  FormControl,
  FormLabel,
  Input,
  Textarea,
  useColorMode,
  useColorModeValue,
} from '@chakra-ui/react';
import { useChain } from '@cosmos-kit/react';
import { chainName, CDT_TOKEN_CONTRACT } from '../config';
import { CdtTokenClient } from '../config/cdt_token/cdt_token.client';
import { CreateProposalMsg } from '../config/cdt_token/cdt_token.types';

const ProposalCreationView: React.FC = () => {
  const { colorMode } = useColorMode();
  const bgColor = useColorModeValue('gray.100', 'whiteAlpha.200');
  const { isWalletConnected, getSigningCosmWasmClient, address } = useChain(chainName);
  const [title, setTitle] = useState('');
  const [description, setDescription] = useState('');
  const [votingOptions, setVotingOptions] = useState('');

  const handleCreateProposal = async () => {
    if (!isWalletConnected || !address) {
      alert('Please connect your wallet first!');
      return;
    }
    try {
      const client = await getSigningCosmWasmClient();
      const cdtTokenClient = new CdtTokenClient(client, address, CDT_TOKEN_CONTRACT);
      const proposalMsg: CreateProposalMsg = {
        title,
        description,
        voting_options: votingOptions.split(','),
      };
      const response = await cdtTokenClient.createProposal(proposalMsg);
      console.log('Create Proposal response:', response);
      alert('Proposal created successfully!');
    } catch (error) {
      console.error('Create Proposal error:', error);
      alert('Creating proposal failed!');
    }
  };

  return (
    <Box bg={bgColor} p={8} borderRadius="lg" boxShadow="md">
      <FormControl id="title" mb={4}>
        <FormLabel>Title</FormLabel>
        <Input
          type="text"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
      </FormControl>
      <FormControl id="description" mb={4}>
        <FormLabel>Description</FormLabel>
        <Textarea
          value={description}
          onChange={(e) => setDescription(e.target.value)}
        />
      </FormControl>
      <FormControl id="votingOptions" mb={4}>
        <FormLabel>Voting Options</FormLabel>
        <Input
          type="text"
          value={votingOptions}
          onChange={(e) => setVotingOptions(e.target.value)}
        />
      </FormControl>
      <Flex justify="center">
        <Button onClick={handleCreateProposal} isDisabled={!isWalletConnected}>
          Create Proposal
        </Button>
      </Flex>
    </Box>
  );
};

export default ProposalCreationView;
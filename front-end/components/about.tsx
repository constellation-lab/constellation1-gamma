import React from 'react';
import { Box, Heading, Text, OrderedList, ListItem, Grid, GridItem, Icon, Flex, useColorMode } from '@chakra-ui/react';
import { FaCheckCircle } from 'react-icons/fa';

export const About = () => {
  const { colorMode } = useColorMode();

  return (
    <Box className="about" p={8} maxW="1200px" mx="auto">
      <Heading as="h1" size="2xl" mb={8} textAlign="center">
        Constella Options
      </Heading>

      <Grid templateColumns={{ base: '1fr', md: '1fr 1fr' }} gap={8} mb={16}>
        <GridItem>
          <Heading as="h4" size="lg" mb={4}>
            What is Constella Options?
          </Heading>
          <Text fontSize="lg" mb={4}>
            Constella Options is a cutting-edge decentralized application (Dapp) built on the Nibiru blockchain, designed to re-imagine and revolutionize options trading in the decentralized finance (DeFi) space. Our platform empowers users to create, buy, sell, and execute options contracts seamlessly with a focus on bespoke custom options and personal price levels, providing a secure and transparent environment for options trading.
          </Text>
          <Text fontSize="lg">
            At Constella Options, we believe in the power of decentralized finance to transform the way people interact with financial instruments. Our mission is to simplify and make options trading accessible to everyone, regardless of their financial background or expertise. By leveraging the security and transparency of the Nibiru blockchain, we aim to create a trustless and efficient options trading ecosystem.
          </Text>
        </GridItem>
        <GridItem>
          <Box
            bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'}
            p={6}
            borderRadius="lg"
            boxShadow="md"
            color={colorMode === 'light' ? 'black' : 'white'}
          >
            <Text fontSize="lg" mb={4}>
              At the simplest level, a Constella Option operates by allowing the option creator to express their confidence in the future direction of an asset by using an Option, which involves:
            </Text>
            <OrderedList spacing={4}>
              <ListItem>
                <Text fontSize="lg">
                  The option creator staking an amount in the contract, called the collateral, which they send to the option owner upon execution of the option.
                </Text>
              </ListItem>
              <ListItem>
                <Text fontSize="lg">
                  In return, the option owner must send a counteroffer amount specified by the options creator (on creation), to receive the collateral, and it must be before the option&apos;s expiry date.
                </Text>
              </ListItem>
            </OrderedList>
          </Box>
        </GridItem>
      </Grid>

      <Heading as="h4" size="xl" mb={8} textAlign="center">
        Key Features
      </Heading>

      <Grid templateColumns={{ base: '1fr', md: 'repeat(2, 1fr)', lg: 'repeat(3, 1fr)' }} gap={8} mb={16}>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Option Creation
            </Text>
          </Flex>
          <Text fontSize="lg">
            Users can easily create options contracts by specifying the collateral amount, counter offer amount, and expiration time. The creator stakes the collateral into the option upon creation, granting them initial ownership.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Market Listing
            </Text>
          </Flex>
          <Text fontSize="lg">
            Option owners can list their options on the market for sale by setting a desired sale price. This allows other users to discover and purchase options that align with their trading strategies.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Option Trading
            </Text>
          </Flex>
          <Text fontSize="lg">
            Buyers can purchase listed options by providing the specified sale price. Upon purchase, the ownership of the option is transferred to the buyer, and the option is automatically removed from the market.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Flexible Ownership
            </Text>
          </Flex>
          <Text fontSize="lg">
            Option owners have the flexibility to update the list price while the option is on the market, remove the option from the market without selling, or transfer ownership to another user.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Option Splitting
            </Text>
          </Flex>
          <Text fontSize="lg">
            Constella Options introduces a unique feature that allows users to split an option into two separate options after creation. Splitting an option can be particularly useful in scenarios where the owner wants to partially sell or execute an option while retaining some exposure to the underlying asset.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Option Execution
            </Text>
          </Flex>
          <Text fontSize="lg">
            Before the expiration time, option owners can execute the option by providing the correct counter offer amount. This action sends the counter offer tokens to the creator and returns the staked collateral to the owner.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Expiration and Claiming
            </Text>
          </Flex>
          <Text fontSize="lg">
            If an option reaches its expiration time, anyone can claim the option, typically the creator. This process returns the staked collateral to the creator and deletes the option from the platform.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Option Burning
            </Text>
          </Flex>
          <Text fontSize="lg">
            Owners have the ability to burn an unexpired option, similar to claiming, but only callable by the owner themselves.
          </Text>
        </GridItem>
        <GridItem>
          <Flex align="center" mb={4}>
            <Icon as={FaCheckCircle} color="green.500" mr={2} />
            <Text fontSize="xl" fontWeight="bold">
              Comprehensive Queries
            </Text>
          </Flex>
          <Text fontSize="lg">
            Constella Options supports a wide range of queries, allowing users to retrieve option details by ID, view all options, filter options by owner or creator address, and access paginated options for efficient data retrieval. (Milestone 2)
          </Text>
        </GridItem>
      </Grid>

      <Heading as="h4" size="xl" mb={8} textAlign="center">
        Top 11 Use Cases
      </Heading>

      <Grid templateColumns={{ base: '1fr', md: 'repeat(2, 1fr)', lg: 'repeat(3, 1fr)' }} gap={8}>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Hedging
            </Heading>
            <Text fontSize="lg">
              Users can create options to hedge their investments in the Nibiru ecosystem. For example, if a user holds a large amount of NIBI tokens and is concerned about potential price fluctuations, they can create put options to protect their holdings.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Speculation
            </Heading>
            <Text fontSize="lg">
              Traders can use the options market to speculate on the future price movements of NIBI and NUSD. By buying or selling options, they can profit from their predictions without having to own the underlying assets.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Yield Enhancement
            </Heading>
            <Text fontSize="lg">
              Users can create options and list them on the market to earn additional income from their holdings. The premium received from selling options can provide a steady stream of passive income.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Arbitrage Opportunities
            </Heading>
            <Text fontSize="lg">
              Traders can exploit price discrepancies between the options market and the spot market to make low-risk profits. This can attract sophisticated traders to the platform.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Collateral Optimization
            </Heading>
            <Text fontSize="lg">
              Users can utilize the options contract to optimize their collateral usage. By creating options with their collateral, they can potentially earn additional income while still maintaining exposure to the underlying assets.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Liquidity Provision
            </Heading>
            <Text fontSize="lg">
              Market makers can use the options contract to provide liquidity to the market. By creating and listing options at various strike prices and expiration dates, they can facilitate trading and earn profits from the bid-ask spread.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Portfolio Diversification
            </Heading>
            <Text fontSize="lg">
              Investors can use options to diversify their portfolios within the Nibiru ecosystem. By incorporating options into their investment strategy, they can potentially reduce risk and enhance returns.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Conditional Trading
            </Heading>
            <Text fontSize="lg">
              The options contract allows users to create conditional trades based on their specific requirements. For example, a user can create an option that only executes if certain price or time conditions are met.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Staking Alternative
            </Heading>
            <Text fontSize="lg">
              Instead of directly using their NIBI tokens, users can create options with their tokens as collateral. This provides an alternative way to earn while maintaining some flexibility with their holdings.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Gamification
            </Heading>
            <Text fontSize="lg">
              The platform intends to introduce gamification elements to encourage user engagement. For example, leaderboards for top option creators or traders, rewards for reaching certain milestones, or contests for predicting market trends can attract users and increase activity on the platform.
            </Text>
          </Box>
        </GridItem>
        <GridItem>
          <Box bg={colorMode === 'light' ? 'gray.100' : 'whiteAlpha.200'} p={6} borderRadius="lg" boxShadow="md" h="100%" color={colorMode === 'light' ? 'black' : 'white'} >
            <Heading as="h5" size="lg" mb={4}>
              Liquid Options
            </Heading>
            <Text fontSize="lg">
              The platform intends to issue users our CNSTE token equivalent matching the collateral value in their option contract, so users can convert &amp; stake this CNSTE token value and earn with the value of the collateral while it&apos;s stuck in their options.
            </Text>
          </Box>
        </GridItem>
      </Grid>
    </Box>
  );
};
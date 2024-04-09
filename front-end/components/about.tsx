import React from 'react';
import { Box, Heading, Text, OrderedList, ListItem } from '@chakra-ui/react';

export const About = () => {
  return (
    <Box p={8} maxW="970px" mx="auto">
      <Heading as="h1" size="2xl" mb={4}>
        Constella Options
      </Heading>

      <Heading as="h4" size="md" mb={4}>
        What is Constella Options?
      </Heading>

      <Text fontSize="lg" mb={4}>
        Constella Options is a cutting-edge decentralized application (Dapp) built on the Nibiru blockchain, designed to re-imagine and revolutionize options trading in the decentralized finance (DeFi) space. Our platform empowers users to create, buy, sell, and execute options contracts seamlessly with a focus on bespoke custom options and personal price levels, providing a secure and transparent environment for options trading.
      </Text>

      <Text fontSize="lg" mb={4}>
        At Constella Options, we believe in the power of decentralized finance to transform the way people interact with financial instruments. Our mission is to simplify and make options trading accessible to everyone, regardless of their financial background or expertise. By leveraging the security and transparency of the Nibiru blockchain, we aim to create a trustless and efficient options trading ecosystem.
      </Text>

      <Text fontSize="lg" mb={8}>
        At the simplest level, a Constella Option operates by allowing the option creator to express their confidence in the future direction of an asset by using an Option, which involves: the option creator staking an amount in the contract, called the collateral, which they send to the option owner upon execution of the option. In return, the option owner must send a counteroffer amount specified by the options creator (on creation), to receive the collateral, and it must be before the option's expiry date.
      </Text>

      <Heading as="h4" size="md" mb={4}>
        Some Constella Options Key Features
      </Heading>

      <OrderedList spacing={4} mb={8}>
        <ListItem>
          <Text fontSize="lg">
            <strong>Option Creation:</strong> Users can easily create options contracts by specifying the collateral amount, counter offer amount, and expiration time. The creator stakes the collateral into the option upon creation, granting them initial ownership.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Market Listing:</strong> Option owners can list their options on the market for sale by setting a desired sale price. This allows other users to discover and purchase options that align with their trading strategies.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Option Trading:</strong> Buyers can purchase listed options by providing the specified sale price. Upon purchase, the ownership of the option is transferred to the buyer, and the option is automatically removed from the market.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Flexible Ownership:</strong> Option owners have the flexibility to update the list price while the option is on the market, remove the option from the market without selling, or transfer ownership to another user.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Option Splitting:</strong> Constella Options introduces a unique feature that allows users to split an option into two separate options after creation. Splitting an option can be particularly useful in scenarios where the owner wants to partially sell or execute an option while retaining some exposure to the underlying asset.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Option Execution:</strong> Before the expiration time, option owners can execute the option by providing the correct counter offer amount. This action sends the counter offer tokens to the creator and returns the staked collateral to the owner.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Expiration and Claiming:</strong> If an option reaches its expiration time, anyone can claim the option, typically the creator. This process returns the staked collateral to the creator and deletes the option from the platform.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Option Burning:</strong> Owners have the ability to burn an unexpired option, similar to claiming, but only callable by the owner themselves.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Comprehensive queries:</strong> Constella Options supports a wide range of queries, allowing users to retrieve option details by ID, view all options, filter options by owner or creator address, and access paginated options for efficient data retrieval. (Milestone 2)
          </Text>
        </ListItem>
      </OrderedList>

      <Heading as="h4" size="md" mb={4}>
        The Top 10 Use Cases that drive usage, activity, and TVL growth:
      </Heading>

      <OrderedList spacing={4}>
        <ListItem>
          <Text fontSize="lg">
            <strong>Hedging:</strong> Users can create options to hedge their investments in the Nibiru ecosystem. For example, if a user holds a large amount of NIBI tokens and is concerned about potential price fluctuations, they can create put options to protect their holdings.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Speculation:</strong> Traders can use the options market to speculate on the future price movements of NIBI and NUSD. By buying or selling options, they can profit from their predictions without having to own the underlying assets.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Yield enhancement:</strong> Users can create options and list them on the market to earn additional income from their holdings. The premium received from selling options can provide a steady stream of passive income.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Arbitrage opportunities:</strong> Traders can exploit price discrepancies between the options market and the spot market to make low-risk profits. This can attract sophisticated traders to the platform.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Collateral optimization:</strong> Users can utilize the options contract to optimize their collateral usage. By creating options with their collateral, they can potentially earn additional income while still maintaining exposure to the underlying assets.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Liquidity provision:</strong> Market makers can use the options contract to provide liquidity to the market. By creating and listing options at various strike prices and expiration dates, they can facilitate trading and earn profits from the bid-ask spread.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Portfolio diversification:</strong> Investors can use options to diversify their portfolios within the Nibiru ecosystem. By incorporating options into their investment strategy, they can potentially reduce risk and enhance returns.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Conditional trading:</strong> The options contract allows users to create conditional trades based on their specific requirements. For example, a user can create an option that only executes if certain price or time conditions are met.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Staking alternative:</strong> Instead of directly using their NIBI tokens, users can create options with their tokens as collateral. This provides an alternative way to earn while maintaining some flexibility with their holdings.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Gamification:</strong> The platform intends to introduce gamification elements to encourage user engagement. For example, leaderboards for top option creators or traders, rewards for reaching certain milestones, or contests for predicting market trends can attract users and increase activity on the platform.
          </Text>
        </ListItem>

        <ListItem>
          <Text fontSize="lg">
            <strong>Liquid Options:</strong> The platform intends to issue users our CNSTE token equivalent matching the collateral value in their option contract, so users can convert & stake this CNSTE token value and earn with the value of the collateral while its stuck in their options.
          </Text>
        </ListItem>
      </OrderedList>
    </Box>
  );
};
import Head from 'next/head';
import {
  Box,
  Divider,
  Grid,
  Container,
  Link,
  Button,
  Flex,
  Icon,
  useColorMode,
  useColorModeValue,
  GridItem,
  Center,
  Tabs,
  TabList,
  Tab,
  TabPanel,
  TabPanels
} from '@chakra-ui/react';
import { BsFillMoonStarsFill, BsFillSunFill } from 'react-icons/bs';
import { CreatedOptionsList, CreateOption, MarketOptionsList, OwnerOptionList, WalletSection } from '../components';
export default function Home() {
  const { colorMode, toggleColorMode } = useColorMode();

  return (
    <Container maxW="5xl" py={10}>
      <Head>
        <title>Constella options App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Flex justifyContent="end" mb={10}>
      <Center w='1100px' h='40px'  fontWeight='bold' fontSize='50px'>Constella Option</Center>
        <Grid
              templateColumns='repeat(2, 0.1fr)'
              gap={30}
      >

          <GridItem>
              <WalletSection />
          </GridItem>
          <GridItem >
            <Button variant="outline" px={0} onClick={toggleColorMode}>
              <Icon
                as={colorMode === 'light' ? BsFillMoonStarsFill : BsFillSunFill}
              />
            </Button>
          </GridItem>
      </Grid>
      </Flex>
      <Tabs>
      <TabList>
        <Tab>Create</Tab>
        <Tab>Market</Tab>
        <Tab>Owned Option</Tab>
        <Tab>Created Option</Tab>
      </TabList>
      <TabPanels>
        <TabPanel><CreateOption /></TabPanel>
        <TabPanel><MarketOptionsList/></TabPanel>
        <TabPanel><OwnerOptionList/></TabPanel>
        <TabPanel><CreatedOptionsList/></TabPanel>
      </TabPanels>
    </Tabs>
    </Container>
  );
}

import Head from 'next/head';
import {
  Grid,
  Container,
  Button,
  Flex,
  Icon,
  useColorMode,
  GridItem,
  Center,
  Tabs,
  TabList,
  Tab,
  TabPanel,
  TabPanels,
  Modal,
  ModalOverlay,
  ModalContent,
  ModalHeader,
  ModalFooter,
  ModalBody,
  useDisclosure,
} from '@chakra-ui/react';
import { BsFillMoonStarsFill, BsFillSunFill } from 'react-icons/bs';
import { CreatedOptionsList, CreateOption, MarketOptionsList, OwnerOptionList, WalletSection } from '../components';
import React, { useEffect } from 'react';
import { inputType } from '../components/types';

function RiskWarningModal() {
  const { isOpen, onOpen, onClose } = useDisclosure();

  useEffect(() => {
    onOpen(); // 当组件挂载时打开模态框
  }, [onOpen]);

  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered>
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>Risks Warning</ModalHeader>
          <ModalBody>
          Trading options involves risk and may not be suitable for every investor. The valuation of options may fluctuate, and as a result. The impact of seasonal and geopolitical events is already factored into market prices.  you should carefully consider your investment objectives, level of experience, and risk appetite. There is always a possibility of losing some or all of your initial investment; hence you should not invest money that you cannot afford to lose. You should be aware of all the risks associated with options trading and seek advice from an independent financial advisor if you have any doubts.

The website and its affiliates take no responsibility for any loss or damage. We do not provide investment advice, and the information on this website should not be construed as such. All content on this site is provided for informational and educational purposes only and is not intended as financial advice.

          </ModalBody>
          <ModalFooter>
            <Button colorScheme="blue" mr={3} onClick={onClose}>
              I have read the warning & Accept it
            </Button>
          </ModalFooter>
        </ModalContent>
      </Modal>
    </>
  );
}


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
        <RiskWarningModal/>
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

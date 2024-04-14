import Head from 'next/head';
import {
  Container,
  Button,
  Flex,
  useColorMode,
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
  Text,
  Box,
} from '@chakra-ui/react';
import { CreatedOptionsList, CreateOption, MarketOptionsList, OwnerOptionList, WalletSection } from '../components';
import React, { useEffect } from 'react';
import { About } from '../components';

function RiskWarningModal() {
  const { isOpen, onOpen, onClose } = useDisclosure();

  useEffect(() => {
    onOpen(); // 当组件挂载时打开模态框
  }, [onOpen]);

  return (
    <>
      <Modal isOpen={isOpen} onClose={onClose} isCentered size="xl">
        <ModalOverlay />
        <ModalContent>
          <ModalHeader>
            <Text fontSize="2xl" fontWeight="bold">
              Risk Warning
            </Text>
          </ModalHeader>
          <ModalBody>
            <Text fontSize="lg" mb={4}>
              Creating and Trading options involves risk and may not be suitable for every investor. The valuation of options may fluctuate as a result of many factors, like the other options in the market, hence this value may fluctuate irrespective of the value you may assign your own created option. You should carefully consider your investment objectives, level of experience, and risk appetite. There is always a possibility of losing some or all of your initial investment; hence you should not invest money that you cannot afford to lose. You should be aware of all the risks associated with options trading and seek advice from an independent financial advisor if you have any doubts.
            </Text>
            <Text fontSize="lg" mb={4}>
              The website and its affiliates take no responsibility for any loss or damage. We do not provide investment advice, and the information on this website should not be construed as such. All content on this site is provided for informational and educational purposes only and is not intended as financial advice.
            </Text>
            <Text fontSize="lg">
              Lastly, participants from the U.S., Canada, Mexico, OFAC-sanctioned countries (Cuba, Iran, North Korea, Syria, Crimea, certain Ukrainian regions), and other sanctioned countries are ineligible to use this app, and hence should close and leave the app.
            </Text>
          </ModalBody>
          <ModalFooter>
            <Button colorScheme="blue" size="lg" onClick={onClose}>
              I have read and accept the risks and eligibility
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
    <Box className="home">
      <Head>
        <title>Constella Options App</title>
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <Container maxW="7xl" py={10}>
        <Flex justify="space-between" align="center" mb={10}>
          <Text fontSize="3xl" fontWeight="bold">
            Constella Options
          </Text>
          <Flex align="center">
            <Button onClick={toggleColorMode} mr={4}>
              {colorMode === 'light' ? 'Dark Theme' : 'Light Theme'}
            </Button>
            <RiskWarningModal />
            <WalletSection />
          </Flex>
        </Flex>

        <Tabs variant="soft-rounded" colorScheme="blue">
          <TabList mb={6}>
            <Tab _selected={{ bg: 'blue.500', color: 'white' }}>Create</Tab>
            <Tab _selected={{ bg: 'blue.500', color: 'white' }}>Market</Tab>
            <Tab _selected={{ bg: 'blue.500', color: 'white' }}>Owned Options</Tab>
            <Tab _selected={{ bg: 'blue.500', color: 'white' }}>Created Options</Tab>
            <Tab _selected={{ bg: 'blue.500', color: 'white' }}>About</Tab>
          </TabList>

          <TabPanels>
            <TabPanel>
              <CreateOption />
            </TabPanel>
            <TabPanel>
              <MarketOptionsList />
            </TabPanel>
            <TabPanel>
              <OwnerOptionList />
            </TabPanel>
            <TabPanel>
              <CreatedOptionsList />
            </TabPanel>
            <TabPanel>
              <About />
            </TabPanel>
          </TabPanels>
        </Tabs>
      </Container>
    </Box>
  );
}
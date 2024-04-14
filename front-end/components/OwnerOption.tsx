import { chainName, contractAddress } from "../config";
import { useChain } from "@cosmos-kit/react";
import {
  Box,
  Button,
  Slider,
  SliderTrack,
  Skeleton,
  SliderThumb,
  VStack,
  useColorModeValue,
  Flex,
  Text,
  Popover,
  PopoverTrigger,
  PopoverContent,
  PopoverHeader,
  PopoverArrow,
  PopoverCloseButton,
  PopoverFooter,
  PopoverBody,
  Input,
  SliderFilledTrack,
  Tooltip,
} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndData } from "../config/constellation";
import { useState } from "react";
import { Data } from "../config/constellation/Constellation.types";
import React from "react";
import { ExecuteMsg } from "../config/constellation/Constellation.types";
import { Coin } from "@cosmjs/amino";
import { useTx } from "../hook";

const TransferButton = ({
    id,
    expires
  }: {
    id:number;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { address, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleTransferOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {transfer:{
            id: id,
            to: toAddr
        }}
        console.log(msg)
        await tx(msg,{});
        setIsSubmitting(false)
      }
  

    const validateAddr =()=>{
        if (toAddr.length != 43){
            return true
        }
        const Regex =/^nibi/;
        const iserror = toAddr.match(Regex);
        if (iserror === null) {
            return true;
        }
        return false
    }
    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)}>Transfer</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
              Option ID: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            Transfer the Option to:
            <Input variant='flushed' isInvalid={validateAddr()} placeholder='Put Address here...' onChange={(input)=>{
                const value = input.target.value;
                setToAddr(value);
            }}/>
            {validateAddr()?(  <Text fontSize='xs' color={"tomato"}>invalid address</Text>):(<></>)}

            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleTransferOption();}} isDisabled = {validateAddr()}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}

const ExeButton = ({
    id,
    data
  }: {
    id:number;
    data:Data
})=>{
    const initialFocusRef = React.useRef()
    const { address, assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useTx(chainName,"unibi",contractAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleExecuteOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {execute:{
            id:id
        }}
        const funds:Coin[]=[data.counter_offer]
        console.log(msg)
        await tx(msg,{},funds);
        setIsSubmitting(false)
      }
  

    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000)}>Execute</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            You will execute the option with ID: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text> you will get:</Text>
                  <Text>
                    {Number(data.collateral.amount)/1000000} 
                    {getdenomMap().get(data.collateral.denom)}
                  </Text>
                </Flex> 
                <Flex justify="space-between" w = "full"><Text> you need to pay:</Text>
                  <Text>
                    {Number(data.counter_offer.amount)/1000000} 
                    {getdenomMap().get(data.counter_offer.denom)}
                  </Text>
                </Flex> 
           </VStack>        

            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleExecuteOption();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}



const SplitButton = ({
    id,
    data,
    expires
  }: {
    id:number;
    data:Data;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const {tx} = useTx(chainName,"unibi",contractAddress)

    const handleSplit = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            split:{
                id:id,
                percentage: price,  
            }
        }
        console.log(msg)
        await tx(msg,{});
        setIsSubmitting(false)
      }
  

    return (
        <Popover
          initialFocusRef={initialFocusRef}
          placement='bottom'
        >{({ isOpen, onClose }) => (
        <>
          <PopoverTrigger>
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)} >Split</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
           <PopoverHeader pt={4} fontWeight='bold' border='10' >
              Please input the percentage you want to split:
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            Set percentage {price}:
            <Box flex={1}>
              <Slider
                flex='1'
                focusThumbOnChange={false}
                value={price}
                onChange={setPrice}
              >
                <SliderTrack>
                  <SliderFilledTrack />
                </SliderTrack>
              <SliderThumb fontSize='sm' boxSize='32px' />
            </Slider>
            </Box>
            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleSplit();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}
const OptionCard = ({ data, id }: { data: Data; id: number }) => {
  const { assets, address } = useChain(chainName);

  const getDenomMap = () => {
    const map = new Map<string, string>();
    assets.assets.forEach((value) => {
      map.set(value.denom_units[0].denom, value.name);
    });
    return map;
  };

  return (
    <Box className="option-card">
      <Flex justify="space-between" align="center" mb={4}>
        <Text fontSize="xl" fontWeight="bold">
          Option ID: {id}
        </Text>
        <Flex>
          <TransferButton id={id} expires={Number(data.expires)} />
          <ExeButton id={id} data={data} />
          <SplitButton id={id} data={data} expires={Number(data.expires)} />
        </Flex>
      </Flex>

      <VStack align="start" spacing={4}>
        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Collateral:</Text>
          <Text>
            {address ? (
              `${Number(data.collateral.amount) / 1000000} ${getDenomMap().get(data.collateral.denom)}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Counter Offer:</Text>
          <Text>
            {address ? (
              `${Number(data.counter_offer.amount) / 1000000} ${getDenomMap().get(data.counter_offer.denom)}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between" w="full">
          <Text fontWeight="bold">Expiration Date:</Text>
          <Text>{new Date(Number(data.expires) / 1000000).toDateString()}</Text>
        </Flex>
      </VStack>
    </Box>
  );
};

export const OwnerOptionList = () => {
  const { address, getCosmWasmClient } = useChain(chainName);
  const [datas, setData] = useState<ArrayOfTupleOfUint64AndData>();

  const handleQueryOwnerList = async () => {
    if (!address) {
      alert("Please connect your wallet!");
      return;
    }

    let client: Awaited<ReturnType<typeof getCosmWasmClient>>;
    try {
      client = await getCosmWasmClient();
    } catch (e: any) {
      console.error(e);
      return;
    }

    const options: Promise<ArrayOfTupleOfUint64AndData> = client.queryContractSmart(contractAddress, {
      owner_options: {
        addr: address,
      },
    });

    console.log(options);
    options.then((value) => {
      setData(value);
      console.log(value);
    });
  };

  return (
    <Box className="owner-options-list">
      <Flex justify="space-between" align="center" mb={6}>
        <Text fontSize="2xl" fontWeight="bold">
          Your Options
        </Text>
        <Button onClick={handleQueryOwnerList} colorScheme="blue" size="lg">
          Refresh
        </Button>
      </Flex>

      {datas ? (
        <VStack spacing={6}>
          {datas.map(([id, data]) => (
            <Box key={id} width="full">
              {!data.isBurned && <OptionCard data={data} id={id} />}
            </Box>
          ))}
        </VStack>
      ) : (
        <Skeleton height={200} />
      )}
    </Box>
  );
};

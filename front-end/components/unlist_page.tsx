import { chainName, MarketAddress } from "../config";
import { useChain } from "@cosmos-kit/react";
import {
  Box,
  Button,
  Skeleton,
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
  Editable,
  EditableInput,
  EditablePreview,
  Tooltip,
} from "@chakra-ui/react";
import { ArrayOfTupleOfUint64AndListItemData, ListItemData, ExecuteMsg } from "../config/market";
import { useState } from "react";
import React from "react";
import { useMarketTx } from "../hook";



const UnlistButton = ({
    id,
    data
  }: {
    id:number;
    data:ListItemData
})=>{
    const initialFocusRef = React.useRef()
    const { address, assets } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [toAddr, setToAddr] = useState("")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }

    const handleUnlistOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            remove_list: {
                id: id
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>Number(data.expires)/1000000)}>Unlist</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
            You are about to unlist the option with id: {id} 
            </PopoverHeader>
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            <VStack align="start" fontWeight="bold" fontSize={{ md: 'lg' }}  mb={1}>
                <Flex justify="space-between" w = "full"><Text>Listed price is:</Text>
                <Text>
                  {address ? (
                    `${Number(data.price)/1000000} NIBI`
                  ) : (
                  <Tooltip label="Connect wallet to see the value" placement="top">
                      <Text cursor="default">-</Text>
                  </Tooltip>
                  )}
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
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleUnlistOption();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}

const UpdatePriceButton = ({
    id,
    data,
    expires
  }: {
    id:number;
    data:ListItemData;
    expires: number;
  })=>{
    const initialFocusRef = React.useRef()
    const { assets, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const [priceToken,setPriceToken] = useState("unibi")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)
    const getdenomMap =() => {
        let map = new Map<String,String>()
        assets.assets.map((value)=>{
            map.set(value.denom_units[0].denom,value.name)
        })
        return map;
    }


    const handleUpdatePrice = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            update_price:{
                id:id,
                price:(price*1000000).toString()
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
            <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now()>expires/1000000)} >Update Price</Button>
          </PopoverTrigger>
          <PopoverContent color='white' bg='blue.800' borderColor='blue.800' >
            {data.price[0]?( <PopoverHeader pt={4} fontWeight='bold' border='10' >
              The Current price is: {Number(data.price)/1000000} NIBI
            </PopoverHeader>):(<PopoverHeader pt={4} fontWeight='bold' border='10' >The Current price is not set</PopoverHeader>)}
            <PopoverArrow bg='blue.800' />
            <PopoverCloseButton />
            <PopoverBody>
            Set price:
            <Box flex={1}>
                <Editable
                variant="unstyled"
                fontSize={{ base: 'lg', sm: 'xl' }}
                fontWeight="bold"
                mb={{ base: 1, sm: 2 }}
                placeholder="0"
                >
                    <EditablePreview />
                    <EditableInput
                    type="number"
                    min="0"
                    defaultValue="0"
                    onChange={(e) => {
                    const value = e.target.value;
                    const floatRegex =
                        /(0{0,1}[.]d*)(d+([.]d*)?(e[+-]?d+)?|[.]d+(e[+-]?d+)?)/g;
                    const floatCheck = value.match(floatRegex);
                    if (floatCheck !== null) {
                        setPrice(Number(value))
                        return value;
                     }
                    setPrice(parseFloat(value))
                    return (e.target.value = parseFloat(value).toString());
                    }}
                    _focus={{ boxShadow: 'none' }}
                />
                </Editable>
            </Box>
            </PopoverBody>

            <PopoverFooter
              border='0'
              display='flex'
              alignItems='center'
              justifyContent='center'
              pb={4}
            >
            <Button colorScheme='blue' ref={initialFocusRef} onClick={()=>{onClose();handleUpdatePrice();}}>
                  Confirm
            </Button>
            </PopoverFooter>
          </PopoverContent>
            </>
          )}
        </Popover>       
    )

}
const OptionCard = ({ data, id }: { data: ListItemData; id: number }) => {
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
          <UnlistButton id={id} data={data} />
          <UpdatePriceButton id={id} data={data} expires={Number(data.expires)} />
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

export const ManageOptionList = () => {
  const { address, getCosmWasmClient } = useChain(chainName);
  const [datas, setData] = useState<ArrayOfTupleOfUint64AndListItemData>();

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

    const options: Promise<ArrayOfTupleOfUint64AndListItemData> = client.queryContractSmart(MarketAddress, {
      owner_list_items: {
        addr: address,
      },
    });

    options.then((value) => {
      setData(value);
      console.log(value);
    });
  };

  return (
    <Box className="manage-options-list">
      <Flex justify="space-between" align="center" mb={6}>
        <Text fontSize="2xl" fontWeight="bold">
          Manage Your Listed Options
        </Text>
        <Button onClick={handleQueryOwnerList} colorScheme="blue" size="lg">
          Refresh
        </Button>
      </Flex>

      {datas ? (
        <VStack spacing={6}>
          {datas.map(([id, data]) => (
            <OptionCard key={id} data={data} id={id} />
          ))}
        </VStack>
      ) : (
        <Skeleton height={200} />
      )}
    </Box>
  );
};

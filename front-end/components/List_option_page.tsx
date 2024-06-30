import { chainName, MarketAddress } from "../config";
import { useChain } from "@cosmos-kit/react";
import {
  Box,
  Button,
  Container,
  List,
  Skeleton,
  Stack,
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
  HStack,
  Select,
  Editable,
  EditableInput,
  EditablePreview,
  Tooltip,
} from "@chakra-ui/react";
import { useState } from "react";
import React from "react";
import { ExecuteMsg, ListItemData, ArrayOfTupleOfUint64AndListItemData } from "../config/market";
import { Coin } from "@cosmjs/amino";
import { useMarketTx } from "../hook";


  const ListMarketButton = ({
    id,
    expires
  }: {
    id: number;
    expires: number;
  }) => {
    const initialFocusRef = React.useRef<HTMLButtonElement>(null);
    const { assets, getCosmWasmClient } = useChain(chainName);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [price, setPrice] = useState(0)
    const [priceToken,setPriceToken] = useState("unibi")
    const {tx} = useMarketTx(chainName,"unibi",MarketAddress)

    const handleExecuteOption = async () => {
        setIsSubmitting(true)
        let msg:ExecuteMsg = {
            list:{
                expires:Math.floor((Date.now()+800*24*60*60*1000)/1000),
                price:(price*1000000).toString(),
                id:id,
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
        >
          {({ isOpen, onClose }) => (
            <>
              <PopoverTrigger>
                <Button isLoading={isSubmitting} colorScheme="primary" isDisabled={(Date.now() > expires / 1000000)}>List Market</Button>
              </PopoverTrigger>
              <PopoverContent color='white' bg='blue.800' borderColor='blue.800'>
            <PopoverHeader pt={4} fontWeight='bold' border='10' >
              List option id: {id} 
            </PopoverHeader>
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
              <Button 
                colorScheme='blue' 
                ref={initialFocusRef} 
                onClick={() => { onClose(); handleExecuteOption(); }}
              >
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
      assets?.assets?.forEach((value) => {
        if (value.denom_units && value.denom_units.length > 0) {
          map.set(value.denom_units[0].denom, value.name);
        }
      });
      return map;
    };

  return (
    <Box className="option-card">
      <Flex justify="space-between" align="center" mb={4}>
        <Text fontSize="xl" fontWeight="bold">
          Option ID: {id}
        </Text>
        <ListMarketButton id={id} expires={Number(data.expires)} />
      </Flex>

      <Stack spacing={4}>
        <Flex justify="space-between">
          <Text fontWeight="bold">Collateral:</Text>
          <Text>
            {address && assets ? (
              `${Number(data.collateral.amount) / 1000000} ${getDenomMap().get(data.collateral.denom) || data.collateral.denom}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between">
          <Text fontWeight="bold">Counter Offer:</Text>
          <Text>
            {address && assets ? (
              `${Number(data.counter_offer.amount) / 1000000} ${getDenomMap().get(data.counter_offer.denom) || data.counter_offer.denom}`
            ) : (
              <Tooltip label="Connect wallet to see the value" placement="top">
                <Text cursor="default">-</Text>
              </Tooltip>
            )}
          </Text>
        </Flex>

        <Flex justify="space-between">
          <Text fontWeight="bold">Expiration Date:</Text>
          <Text>{new Date(Number(data.expires) / 1000000).toDateString()}</Text>
        </Flex>
      </Stack>
    </Box>
  );
};

export const ListOptionList = () => {
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
      owner_un_list_items: {
        addr: address,
      },
    });

    options.then((value) => {
      setData(value);
      console.log(value);
    });
  };

  return (
    <Box className="list-option-list">
      <Flex justify="space-between" align="center" mb={6}>
        <Text fontSize="2xl" fontWeight="bold">
          Your Options
        </Text>
        <Button onClick={handleQueryOwnerList}>Refresh</Button>
      </Flex>

      {datas ? (
        <Stack spacing={6}>
          {datas.map(([id, data]) => (
            <OptionCard key={id} data={data} id={id} />
          ))}
        </Stack>
      ) : (
        <Skeleton height={200} />
      )}
    </Box>
  );
};

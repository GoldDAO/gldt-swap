import { Box, GridItem, HStack, Heading, Text } from '@chakra-ui/react';
import React from 'react';
import { useNft } from '@utils/hooks/gldnfts/useNFTs';
import { useAllCanisters } from '@utils/hooks/useAllCanisters';
import { useTotalSupply } from '@utils/hooks/gldtLedger/useTotalSupply';
import TokenSign from '@ui/gldt/TokenSign';
import GridSystem from '@ui/layout/GridSystem';
import Title from '../layout/Title';

const TransparencyContent = () => {
    const actors = useAllCanisters();
    const { nfts, isLoading } = useNft(actors, 'm45be-jaaaa-aaaak-qcgnq-cai');
    const totalSupply = useTotalSupply();

    const totalWeightSwapped = (totalSupply.gldt / 100).toFixed(2);

    const getTotalWeight = (nfts, w) =>
        nfts.reduce((ac, e) => {
            if (e.weight === w) {
                return ac + e.weight;
            }
            return ac;
        }, 0);

    const arr = [
        getTotalWeight(nfts, 1),
        getTotalWeight(nfts, 10),
        getTotalWeight(nfts, 100),
        getTotalWeight(nfts, 1000),
    ];

    const w = [1, 10, 100, 1000];
    return (
        <GridSystem gap={['0px', '0px', '40px']}>
            <Title title={'GLDT'} subTitle={'Transparency'} />
            <GridItem colSpan={['12', '12', '12']} pt={['20px', '20px', 0]}>
                <Text fontSize={'20px'} width={['100%', '100%', '50%']}>
                    GLDT are minted at a ratio of 100 GLDT per gram of GLD NFT. GLDT is about
                    transparency and let&apos;s everyone verify themselves that the ratio of GLDT to
                    GLD NFT in the swap contract is valid.
                </Text>
            </GridItem>
            <GridItem
                gridColumn={['1/12', '1/12', '1/12']}
                pt={['40px', '40px', 0]}
                pb={['20px', '20px', 0]}
            >
                <Heading
                    fontWeight={300}
                    as="h3"
                    fontSize={'16px'}
                    textAlign={'left'}
                    w={'100%'}
                    borderBottom="1px"
                    borderBottomColor={'secondaryText'}
                >
                    Overview
                </Heading>
                <Text fontSize={'16px'} pt="20px" width={['100%', '100%', '100%', '50%']}>
                    {`The "Total Supply of GLDT" should always be smaller than or equal to the "Total
                    GLD NFTs Swapped" times 100. The total supply may be smaller due to the minor
                    transaction fee of 0.0001 GLDT or because GLDT were burned without actually
                    swapping GLD NFTs. In the end, there is always at least the same amount of GLD
                    NFTs swapped as the equivalent amount of GLDT minted.`}
                </Text>
            </GridItem>
            <GridItem colSpan={['12', '12', '6', '3']} py={['10px', '10px', '20px']}>
                <Text fontSize={'14px'} fontWeight={500}>
                    Total Supply
                </Text>
                <HStack fontSize={'34px'} fontWeight={300}>
                    <Text fontWeight={300} fontSize={'inherit'} fontFamily={'inter'}>
                        {parseInt(totalSupply.gldt)}
                    </Text>
                    <Box fontSize={'18px'}>
                        <TokenSign />
                    </Box>
                </HStack>
            </GridItem>
            <GridItem colSpan={['6', '6', '6', '8']} py={['10px', '10px', '20px']}>
                <Text fontSize={'14px'} fontWeight={500}>
                    GLD NFTs Total Swapped
                </Text>
                <HStack fontSize={'34px'}>
                    <Text fontSize={'inherit'} fontWeight={'200'} fontFamily={'inter'}>
                        {parseInt(totalWeightSwapped)}
                        <Box as="span" fontSize={'18px'}>
                            g
                        </Box>
                    </Text>
                </HStack>
            </GridItem>
            <GridItem
                gridColumn={['1/12', '1/12', '1/12']}
                pt={['40px', '40px', 0]}
                pb={['20px', '20px', 0]}
            >
                <Heading
                    fontWeight={300}
                    as="h3"
                    fontSize={'16px'}
                    textAlign={'left'}
                    w={'100%'}
                    borderBottom="1px"
                    borderBottomColor={'secondaryText'}
                >
                    NFTs supply Breakdown
                </Heading>
                <Text fontSize={'16px'} pt="20px" width={['100%', '100%', '100%', '50%']}>
                    {`Below is the individual breakdown of the sizes of GLD NFTs swapped. These will
                    add up to the value of "Total GLD NFTs Swapped".`}
                </Text>
            </GridItem>
            {arr.map((e, i) => (
                <GridItem
                    colSpan={['6', '6', '6', '3']}
                    key={i}
                    fontSize={'34px'}
                    py={['10px', '10px', '20px']}
                >
                    <Text fontSize={'14px'} fontWeight={500}>
                        {w[i]}g GLD Nfts
                    </Text>
                    <Text
                        fontSize={'inherit'}
                        fontFamily={'inter'}
                        fontWeight={'200'}
                        w={['100%', '100%', '70%']}
                        borderRight={[0, 0, '1px']}
                        borderColor={'secondaryText'}
                    >
                        {e}
                        <Box as="span" fontSize={'18px'}>
                            g
                        </Box>
                    </Text>
                </GridItem>
            ))}
        </GridSystem>
    );
};

export default TransparencyContent;

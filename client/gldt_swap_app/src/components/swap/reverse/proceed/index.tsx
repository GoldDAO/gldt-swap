import { useQueryClient } from "@tanstack/react-query";
import { ArrowDownIcon } from "@heroicons/react/20/solid";

import { useNft, useReverseSwapProceed } from "@context/index";

import { Dialog, Button } from "@components/ui";
import TransactionDetails from "../TransactionDetails";

import Pending from "./Pending";
import Success from "./Success";
import Error from "./Error";

const Proceed = () => {
  const {
    getSelectedTotal,
    getSelectedTotalGLDT,
    getSelectedTotalGram,
    resetState,
  } = useNft();
  const totalNFTs = getSelectedTotal();
  const totalGLDNFTtoSwap = getSelectedTotalGram();
  const totalGLDTtoSwap = getSelectedTotalGLDT();
  const queryClient = useQueryClient();
  const {
    state: reverseSwapProceedState,
    handleShow,
    handleClose,
    reverseSwap,
    setCanCloseDialog,
  } = useReverseSwapProceed();
  const { show, canReverseSwap } = reverseSwapProceedState;
  const {
    mutate: mutateSwapGLDNFT,
    isSuccess,
    isError,
    isPending,
    isIdle,
  } = reverseSwap;

  const handleOnClick = () => {
    mutateSwapGLDNFT(undefined, {
      onSuccess: () => {
        setCanCloseDialog(true);
        queryClient.invalidateQueries({
          queryKey: ["GET_AVAILABLE_GLD_NFT_1G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["GET_AVAILABLE_GLD_NFT_10G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["GET_AVAILABLE_GLD_NFT_100G"],
        });
        queryClient.invalidateQueries({
          queryKey: ["GET_AVAILABLE_GLD_NFT_1000G"],
        });
        resetState();
      },
    });
  };

  return (
    <>
      <Button
        onClick={handleShow}
        className={`rounded-xl w-full py-3`}
        disabled={!canReverseSwap}
      >
        Preview Swap
      </Button>
      <Dialog show={show} handleClose={handleClose} enableClose={isIdle}>
        <div className="px-6 pt-6 pb-12">
          {isIdle && (
            <>
              <div className="px-6 mb-8 text-center">
                You are sending{" "}
                <span className="font-semibold text-gold">
                  {totalGLDTtoSwap + totalNFTs} GLDT
                </span>{" "}
                and will receive{" "}
                <span className="font-semibold text-gold">
                  {totalGLDNFTtoSwap}g GLD NFTs
                </span>
                . <br />
                <span className="font-semibold text-gold">
                  {totalGLDTtoSwap} GLDT
                </span>{" "}
                will be burned and{" "}
                <span className="font-semibold text-gold">
                  {totalNFTs} GLDT
                </span>{" "}
                fee are charged.
              </div>
              <div className="flex flex-col items-center gap-6 border border-border bg-surface-2 p-6 rounded-xl">
                <div className="font-semibold">
                  {totalGLDTtoSwap + totalNFTs} GLDT
                </div>

                <div className="w-full flex justify-center items-center py-4">
                  <div className="relative w-full">
                    <div className="border-t border-border w-full"></div>
                    <div className="absolute inset-x-0 top-0 flex justify-center transform -translate-y-1/2">
                      <button className="bg-content text-background rounded-full p-2 cursor-default">
                        <ArrowDownIcon
                          height={24}
                          width={24}
                          className="text-gold"
                        />
                      </button>
                    </div>
                  </div>
                </div>
                <div className="font-semibold">
                  {totalGLDNFTtoSwap}g of gold
                </div>
              </div>

              <TransactionDetails className="w-full mt-8" />

              <div className="text-center mt-8">
                <Button onClick={handleOnClick}>Confirm</Button>
              </div>
            </>
          )}
          {isPending && <Pending />}
          {isSuccess && <Success />}
          {isError && <Error />}
        </div>
      </Dialog>
    </>
  );
};

export default Proceed;

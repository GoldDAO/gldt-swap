import clsx from "clsx";
import {
  GLDT_VALUE_1G_NFT,
  GLDT_LEDGER_CANISTER_ID,
  GLDT_STAKE_CANISTER_ID,
} from "@constants";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchUserTotalStaked from "@services/gldt_stake/hooks/useFetchUserTotalStaked";
import useFetchTokenPrice from "@hooks/useFetchTokenPrice";
import useFetchStakeAPY from "@services/gldt_stake/hooks/useFetchStakeAPY";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const StakeOverview = () => {
  const { authenticatedAgent, unauthenticatedAgent, isConnected } = useAuth();

  const fetchUserTotalStaked = useFetchUserTotalStaked(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    {
      enabled: isConnected && !!authenticatedAgent,
    }
  );

  const tokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: "GLDT",
    from_canister_id: GLDT_LEDGER_CANISTER_ID,
    amount: fetchUserTotalStaked.data ?? 0n,
    enabled:
      !!unauthenticatedAgent && isConnected && fetchUserTotalStaked.isSuccess,
  });

  const fetchStakeAPY = useFetchStakeAPY(
    GLDT_STAKE_CANISTER_ID,
    unauthenticatedAgent,
    {
      enabled: !!unauthenticatedAgent,
    }
  );

  const renderTotalActiveStake = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            <TokenValueToLocaleString
              value={tokenPrice.data.amount}
              tokenDecimals={tokenPrice.data.decimals}
            />
            <div className="text-content/60 font-normal">GLDT</div>
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return (
        <>
          0<div className="text-content/60 font-normal">GLDT</div>
        </>
      );
    }
  };

  const renderTotalActiveStakePrice = () => {
    if (isConnected) {
      if (tokenPrice.isSuccess) {
        return (
          <>
            <TokenValueToLocaleString
              value={tokenPrice.data.amount / BigInt(GLDT_VALUE_1G_NFT)}
              tokenDecimals={tokenPrice.data.decimals}
            />{" "}
            grams of Gold ({" "}
            <span>
              $<NumberToLocaleString value={tokenPrice.data.amount_usd} />
            </span>
            )
          </>
        );
      } else {
        return "Loading...";
      }
    } else {
      return <>0 grams of Gold ($0)</>;
    }
  };

  return (
    <div
      className={clsx(
        "bg-linear-to-t from-neutral-100 to-background dark:from-neutral-900 dark:to-neutral-800",
        "rounded-tr-[inherit] px-4 xl:px-8 pt-4 xl:pt-8 pb-24"
      )}
    >
      <div className="flex flex-col items-center">
        <div className="grid grid-cols-1 xl:grid-cols-3 w-full">
          <div></div>
          <div className="flex justify-center items-center gap-2">
            <Logo name="gldt" className="h-8 w-8" />
            <div>GLDT</div>
          </div>
          <div className="flex justify-center items-center xl:justify-end mt-2 xl:mt-0">
            <div className="px-4 py-1 text-sm bg-secondary text-white/90 rounded-full">
              Current APY:{" "}
              <span>
                {fetchStakeAPY.isSuccess ? (
                  <>
                    <NumberToLocaleString
                      value={fetchStakeAPY.data}
                      decimals={1}
                    />
                    {"%"}
                  </>
                ) : (
                  "Loading..."
                )}
              </span>
            </div>
          </div>
        </div>
        <div className="py-8">
          <div className="flex flex-col items-center gap-2">
            <div>
              <div className="font-semibold flex flex-col items-center gap-2">
                <div>Total active stake</div>
                <div className="text-2xl xl:text-4xl  flex items-center gap-2">
                  {renderTotalActiveStake()}
                </div>
              </div>
            </div>
            <div className="text-sm text-content/60">
              {renderTotalActiveStakePrice()}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default StakeOverview;

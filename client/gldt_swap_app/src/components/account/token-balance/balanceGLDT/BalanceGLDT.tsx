import { LoaderSpin } from "@components/ui";

import { useUserBalanceGLDT } from "@hooks/gldt_ledger";

const BalanceGLDT = ({ className }: { className?: string }) => {
  const { data: balance, isSuccess, isError, isLoading } = useUserBalanceGLDT();
  return (
    <div className={`${className}`}>
      <div className="border border-border rounded-xl bg-surface p-6">
        {isSuccess && balance && (
          <div className="flex flex-col sm:flex-row items-center justify-center sm:justify-start gap-4">
            <div className="flex items-center justify-center sm:justify-start gap-3">
              <img className="flex-none h-8" src={`/gldt_logo.svg`} />
              <div className="font-semibold text-2xl">{balance.string}</div>
              <div className="font-semibold text-2xl">GLDT</div>
            </div>
            <div className="font-light text-content/60">= $</div>
          </div>
        )}
        {(isLoading || isError) && (
          <div className="flex justify-center">
            <LoaderSpin />
          </div>
        )}
      </div>
    </div>
  );
};

export default BalanceGLDT;
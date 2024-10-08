import { Bounce, toast } from 'react-toastify';

import { useState } from 'react';
import { hexStringToUint8Array } from '@dfinity/utils';
import { p } from '../utils/parsers';
import useActor from './useActor';
import useSession from './useSession';

const useTransfer = ({ selectedToken, amount, to }) => {
  const [loading, setLoading] = useState(false);
  const { principal, isConnected } = useSession();
  const [token] = useActor(selectedToken);

  const isIdStringPrincipal = to.includes('-');
  const isICP = selectedToken === 'icp';

  const icrc1Transfer = async () => {
    if (!isConnected) {
      toast.error('you must be logged.', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      return console.log('you must be logged.');
    }
    if (!token) {
      toast.error('you must be logged!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
      return console.log('selected token is wrong.');
    }
    setLoading(true);
    try {
      let toAddress;
      if (isIdStringPrincipal && isICP) {
        toAddress = await token.account_identifier({ owner: p(to), subaccount: [] });
      } else if (!isIdStringPrincipal && isICP) {
        toAddress = hexStringToUint8Array(to);
      } else {
        toAddress = to;
      }
      const balance = Number(await token.icrc1_balance_of({ owner: p(principal), subaccount: [] }));
      const fee = Number(await token.icrc1_fee());
      const decimals = Number(await token.icrc1_decimals());

      const parsedAmount = parseInt(amount * 10 ** decimals, 10);
      if (balance < parsedAmount + fee) {
        setLoading(false);

        return toast.error('Insufficient balance', {
          position: 'top-right',
          autoClose: 5000,
          hideProgressBar: false,
          closeOnClick: true,
          pauseOnHover: true,
          draggable: true,
          progress: undefined,
          theme: 'light',
          transition: Bounce,
        });
      }
      let tx;
      if (isICP) {
        tx = await token.transfer({
          to: toAddress,
          fee: { e8s: 10000 },
          memo: BigInt(0),
          from_subaccount: [],
          created_at_time: [],
          amount: { e8s: parsedAmount },
        });
      } else {
        tx = await token.icrc1_transfer({
          to: { owner: p(toAddress), subaccount: [] },
          fee: [],
          memo: [],
          from_subaccount: [],
          created_at_time: [],
          amount: parsedAmount,
        });
      }

      if (tx.Ok) {
        setLoading(false);
        return toast.success('Transaction successful!', {
          position: 'top-right',
          autoClose: 5000,
          hideProgressBar: false,
          closeOnClick: true,
          pauseOnHover: true,
          draggable: true,
          progress: undefined,
          theme: 'light',
          transition: Bounce,
        });
      }
      setLoading(false);
      return toast.error('unexpected error!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
    } catch (err) {
      setLoading(false);
      return toast.error('unexpected error!', {
        position: 'top-right',
        autoClose: 5000,
        hideProgressBar: false,
        closeOnClick: true,
        pauseOnHover: true,
        draggable: true,
        progress: undefined,
        theme: 'light',
        transition: Bounce,
      });
    }
  };

  return { icrc1Transfer, loading };
};

export default useTransfer;

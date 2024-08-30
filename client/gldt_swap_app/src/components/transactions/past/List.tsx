/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { useMemo } from "react";
import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { EyeIcon } from "@heroicons/react/24/outline";
// import CopyToClipboard from "@components/buttons/CopyToClipboard";
import { Table, LoaderSpin } from "@components/ui";
import TransactionStatus from "@components/transactions/badge/TransactionStatus";

import { TableProps } from "@utils/table/useTable";

import { useGetUserHistoricSwap } from "@hooks/gldt_swap";
import ListEmptyItem from "./Empty";
import { SwapData } from "@canisters/gldt_swap/interfaces";

const List = ({
  pagination,
  setPagination,
  sorting,
  setSorting,
}: TableProps) => {
  const navigate = useNavigate();

  const columns = useMemo<ColumnDef<SwapData>[]>(
    () => [
      {
        accessorKey: "label",
        id: "label",
        cell: ({ getValue }) => (
          <div className="font-semibold">{getValue()}</div>
        ),
        header: "Type",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "created_at",
        id: "created_at",
        cell: ({ getValue }) => <div>{getValue()}</div>,
        header: "Created at",
        meta: {
          className: "text-left",
        },
      },
      {
        cell: ({ row }) => (
          <div className="font-semibold">
            {" "}
            {row.original.type === "forward" && (
              <div>{row.original.gldt_value} GLDT</div>
            )}
            {row.original.type === "reverse" && (
              <div>{row.original.nft_value}g of GOLD</div>
            )}
          </div>
        ),
        header: "Swapping",
      },
      {
        cell: ({ row }) => (
          <div className="font-semibold">
            {" "}
            {row.original.type === "forward" && (
              <div>{row.original.nft_value}G of GOLD</div>
            )}
            {row.original.type === "reverse" && (
              <div>{row.original.gldt_value} GLDT</div>
            )}
          </div>
        ),
        header: "Receiving",
      },
      {
        accessorKey: "status",
        id: "status",
        cell: ({ getValue }) => (
          <div className="flex justify-center">
            <TransactionStatus status={getValue().label} />
          </div>
        ),
        header: "Status",
      },
      {
        header: "View",
        cell: (props) => (
          <div className="flex justify-center items-center shrink-0 rounded-full bg-surface border border-border hover:bg-surface-2 w-10 h-10">
            <button onClick={() => handleClickView(props)}>
              <EyeIcon className="h-5 w-5" />
            </button>
          </div>
        ),
        meta: {
          className: "text-center",
        },
      },
    ],
    // eslint-disable-next-line react-hooks/exhaustive-deps
    []
  );

  const history_swap = useGetUserHistoricSwap({
    refetchInterval: 10000,
    limit: pagination?.pageSize as number,
    page: pagination.pageIndex as number,
  });

  const handleClickView = (cell) => {
    navigate(
      `/swap/account/transactions/${cell?.row?.original?.nft_id}?index=${cell?.row?.original?.index}`
    );
  };

  return (
    <div>
      {history_swap.isSuccess &&
        history_swap.data &&
        history_swap.data.rows.length !== 0 && (
          <Table
            columns={columns}
            data={history_swap.data}
            pagination={pagination}
            setPagination={setPagination}
            sorting={sorting}
            setSorting={setSorting}
          />
        )}
      {history_swap.isSuccess &&
        history_swap.data &&
        history_swap.data.rows.length === 0 && <ListEmptyItem />}
      {history_swap.isLoading && (
        <div className="flex items-center justify-center h-40">
          <LoaderSpin size="xl" />
        </div>
      )}
      {history_swap.isError && (
        <div className="flex items-center justify-center h-40 text-red-500 font-semibold">
          <div>{history_swap.error?.message}</div>
        </div>
      )}
    </div>
  );
};

export default List;
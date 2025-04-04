/* eslint-disable @typescript-eslint/ban-ts-comment */
// @ts-nocheck
import { ReactNode, useMemo, Fragment } from "react";
import { useSearchParams } from "react-router-dom";
import {
  useReactTable,
  getCoreRowModel,
  getSortedRowModel,
  getExpandedRowModel,
  getFilteredRowModel,
  getPaginationRowModel,
  flexRender,
  ColumnDef,
  PaginationState,
  OnChangeFn,
  SortingState,
} from "@tanstack/react-table";
import {
  ArrowDownIcon,
  ArrowUpIcon,
  ChevronDoubleRightIcon,
  ChevronDoubleLeftIcon,
  ChevronLeftIcon,
  ChevronRightIcon,
} from "@heroicons/react/20/solid";
import { SelectTablePageLimit } from "@components/ui/select/index";

interface ReactTableProps<T extends object> {
  data: T;
  columns: ColumnDef<T.rows>[];
  pagination?: PaginationState;
  setPagination?: OnChangeFn<PaginationState>;
  sorting?: SortingState;
  setSorting?: OnChangeFn<SortingState>;
  getRowCanExpand?: (row: Row<T.rows>) => boolean;
  subComponent?: ReactNode;
  identifier?: string;
  serverSide?: boolean;
}

const linesPerPageOptions = [
  { value: 5 },
  { value: 20 },
  { value: 50 },
  { value: 100 },
];

const Table = <T extends object>({
  columns,
  data,
  pagination,
  setPagination,
  sorting,
  setSorting,
  getRowCanExpand,
  identifier = "",
  subComponent,
  serverSide = true,
}: ReactTableProps<T>) => {
  const [searchParams, setSearchParams] = useSearchParams();
  const pageIndex = `page_index${identifier ?? `_${identifier}`}`;
  const pageSize = `page_size${identifier ?? `_${identifier}`}`;

  const defaultData = useMemo(() => [], []);

  const table = useReactTable({
    data: data?.rows ?? defaultData,
    columns,
    state: {
      pagination,
      sorting,
    },
    onSortingChange: setSorting,
    onPaginationChange: setPagination,
    getCoreRowModel: getCoreRowModel(),
    getRowCanExpand,
    getExpandedRowModel: getExpandedRowModel(),
    ...(serverSide && {
      rowCount: data?.rowCount ?? 0,
      manualPagination: setPagination ? true : undefined,
      manualSorting: setSorting ? true : undefined,
    }),
    ...(!serverSide && {
      getSortedRowModel: getSortedRowModel(),
      getFilteredRowModel: getFilteredRowModel(),
      getPaginationRowModel: getPaginationRowModel(),
      onPaginationChange: setPagination,
    }),
  });

  const handleOnChangePageSize = (value: string) => {
    table.setPageSize(Number(value));
    table.setPageIndex(0);
    searchParams.set(pageSize, value);
    searchParams.set(pageIndex, "1");
    setPagination({ pageIndex: 1, pageSize: value });
    setSearchParams(searchParams, { replace: true });
  };

  // const handleOnChangePageIndex = (e) => {
  //   const page = e.target.value ? Number(e.target.value) - 1 : 0;
  //   table.setPageIndex(page);
  //   searchParams.set("pageIndex", (page + 1).toString());
  //   setSearchParams(searchParams);
  // };

  const handleOnClickFirstPage = () => {
    table.firstPage();
    searchParams.set(pageIndex, "1");
    setSearchParams(searchParams, { replace: true });
  };

  const handleOnClickPreviousPage = () => {
    table.previousPage();
    searchParams.set(
      pageIndex,
      table.getState().pagination.pageIndex.toString()
    );
    setSearchParams(searchParams, { replace: true });
  };

  const handleOnClickNextPage = () => {
    table.nextPage();
    searchParams.set(
      pageIndex,
      (table.getState().pagination.pageIndex + 2).toString()
    );
    setSearchParams(searchParams, { replace: true });
  };

  const handleOnClickLastPage = () => {
    table.lastPage();
    searchParams.set(pageIndex, table.getPageCount().toString());
    setSearchParams(searchParams, { replace: true });
  };

  const handleOnChangeSorting = (columnId: string) => {
    // Detect the current sorting state of the column
    const currentSort = table.getColumn(columnId).getIsSorted();
    const newSortDirection =
      currentSort === "asc" ? "desc" : currentSort === "desc" ? null : "asc";
    setSorting([{ id: columnId, desc: newSortDirection === "desc" }]);
    searchParams.set("id", columnId);
    searchParams.set("desc", newSortDirection === "desc");
    setSearchParams(searchParams);
  };

  return (
    <div className="bg-surface border border-border rounded-xl">
      <div className="overflow-x-auto w-full">
        <table className="table-auto w-full rounded-xl">
          <thead className="bg-surface-2 text-content">
            {table.getHeaderGroups().map((headerGroup) => (
              <tr key={headerGroup.id}>
                {headerGroup.headers.map((header) => {
                  return (
                    <th
                      key={header.id}
                      colSpan={header.colSpan}
                      className="py-4 px-8 first:rounded-tl-lg last:rounded-tr-lg font-normal"
                    >
                      {header.isPlaceholder ? null : (
                        <div
                          className={`flex items-center ${
                            setSorting && header.column.getCanSort()
                              ? "cursor-pointer select-none"
                              : ""
                          } ${
                            header.column.columnDef.meta?.className ??
                            "justify-center"
                          }`}
                          onClick={
                            setSorting && header.column.getCanSort()
                              ? () => handleOnChangeSorting(header.id)
                              : null
                          }
                          title={
                            setSorting && header.column.getCanSort()
                              ? header.column.getNextSortingOrder() === "asc"
                                ? "Sort ascending"
                                : "Sort descending"
                              : undefined
                          }
                        >
                          {flexRender(
                            header.column.columnDef.header,
                            header.getContext()
                          )}
                          {{
                            asc: <ArrowUpIcon className="h-5 w-5 ml-2" />,
                            desc: <ArrowDownIcon className="h-5 w-5 ml-2" />,
                          }[header.column.getIsSorted() as string] ?? null}
                        </div>
                      )}
                    </th>
                  );
                })}
              </tr>
            ))}
          </thead>
          <tbody>
            {table.getRowModel().rows.map((row) => {
              return (
                <Fragment key={row.id}>
                  <tr className="bg-surface border-b last:border-none border-border">
                    {row.getVisibleCells().map((cell) => {
                      return (
                        <td
                          key={cell.id}
                          className={`px-8 py-4 overflow-hidden text-ellipsis whitespace-nowrap ${
                            cell.column.columnDef.meta?.className ??
                            "text-center"
                          }`}
                        >
                          {flexRender(
                            cell.column.columnDef.cell,
                            cell.getContext()
                          )}
                        </td>
                      );
                    })}
                  </tr>
                  {row.getIsExpanded() && (
                    <tr>
                      {/* 2nd row is a custom 1 cell row */}
                      <td colSpan={row.getVisibleCells().length}>
                        {subComponent({ row })}
                      </td>
                    </tr>
                  )}
                </Fragment>
              );
            })}
          </tbody>
        </table>
      </div>

      <div className="p-1 w-full">
        {pagination && setPagination && (
          <div className="flex flex-col md:flex-row md:items-center md:justify-between p-6">
            <div className="flex order-last justify-center md:order-first md:justify-start items-center">
              <span>Lines per page</span>
              <SelectTablePageLimit
                options={linesPerPageOptions}
                value={table.getState().pagination.pageSize}
                handleOnChange={(v) => handleOnChangePageSize(v)}
                className="ml-2 w-25"
              />
            </div>

            <div className="flex flex-col md:flex-row md:items-center md:justify-between gap-2">
              <div className="flex justify-center md:justify-start mb-2 md:mb-0">
                <button
                  className="p-1"
                  onClick={handleOnClickFirstPage}
                  disabled={!table.getCanPreviousPage()}
                >
                  <ChevronDoubleLeftIcon className="h-5 w-5" />
                </button>
                <button
                  className="p-1"
                  onClick={handleOnClickPreviousPage}
                  disabled={!table.getCanPreviousPage()}
                >
                  <ChevronLeftIcon className="h-5 w-5" />
                </button>
                <button
                  className="p-1"
                  onClick={handleOnClickNextPage}
                  disabled={!table.getCanNextPage()}
                >
                  <ChevronRightIcon className="h-5 w-5" />
                </button>
                <button
                  className="p-1"
                  onClick={handleOnClickLastPage}
                  disabled={!table.getCanNextPage()}
                >
                  <ChevronDoubleRightIcon className="h-5 w-5" />
                </button>
              </div>

              <div className="flex justify-center md:justify-start mb-4 md:mb-0 items-center gap-1">
                <div>Page</div>
                <strong>
                  {table.getState().pagination.pageIndex + 1} of{" "}
                  {table.getPageCount().toLocaleString()}
                </strong>
              </div>
              {/* <span className="flex items-center gap-1">
            | Go to page:
            <input
              type="number"
              defaultValue={table.getState().pagination.pageIndex + 1}
              onChange={(e) => {
                handleOnChangePageIndex(e);
              }}
              className="border p-1 rounded w-16"
            />
          </span> */}

              {/* <select
        value={table.getState().pagination.pageSize}
        onChange={(e) => {
          table.setPageSize(Number(e.target.value));
        }}
      >
        {[10, 20, 30, 40, 50].map((pageSize) => (
          <option key={pageSize} value={pageSize}>
            Show {pageSize}
          </option>
        ))}
      </select> */}
              {data?.isFetching ? "Loading..." : null}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default Table;

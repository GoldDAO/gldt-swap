import { useState } from "react";
import {
  PaginationState,
  SortingState,
  OnChangeFn,
} from "@tanstack/react-table";
import { useSearchParams } from "react-router-dom";

export interface TableProps {
  pagination?: PaginationState;
  setPagination?: OnChangeFn<PaginationState>;
  sorting?: SortingState;
  setSorting?: OnChangeFn<SortingState>;
}

export const usePagination = ({
  pageSize = 10,
  pageIndex = 0,
  identifier = "",
}): [PaginationState, OnChangeFn<PaginationState>] => {
  const [searchParams] = useSearchParams();
  const _pageSize = Number(
    searchParams.get(`page_size${identifier ?? `_${identifier}`}`)
  );
  const _pageIndex = Number(
    searchParams.get(`page_index${identifier ?? `_${identifier}`}`)
  );
  const [pagination, setPagination] = useState<PaginationState>({
    pageSize: _pageSize ? _pageSize : pageSize,
    pageIndex: _pageIndex ? _pageIndex - 1 : pageIndex,
  });
  return [pagination, setPagination];
};

export const useSorting = ({
  id = "",
  desc = true,
  identifier = "",
}): [SortingState, OnChangeFn<SortingState>] => {
  const [searchParams] = useSearchParams();
  const _id = searchParams.get(`id${identifier ?? `_${identifier}`}`);
  const _desc = searchParams.get(`desc${identifier ?? `_${identifier}`}`);
  const [sorting, setSorting] = useState<SortingState>([
    {
      id: _id ? _id : id,
      desc: _desc ? _desc === "true" : desc,
    },
  ]);
  return [sorting, setSorting];
};

import { atom } from 'jotai';

export const gldNftAtom = atom([{}]);

export const setGetGldNftsAtom = atom(
  (get) => get(gldNftAtom),
  (get, set, nft) => {
    set(gldNftAtom, nft);
  },
);

export const emptyAllNftsAtom = atom(null, (get, set) => {
  set(gldNftAtom, []);
});

// export const nft1Atom = atom([]);

// export const setGetNfts1Atom = atom(
//   (get) => get(nft1Atom),
//   (get, set, nfts) => {
//     set(nft1Atom, nfts);
//   },
// );

// export const nft10Atom = atom([]);

// export const setGetNfts10Atom = atom(
//   (get) => get(nft10Atom),
//   (get, set, nfts) => {
//     set(nft10Atom, nfts);
//   },
// );

// export const nft100Atom = atom([]);

// export const setGetNfts100Atom = atom(
//   (get) => get(nft100Atom),
//   (get, set, nfts) => {
//     set(nft100Atom, nfts);
//   },
// );

// export const nft1000Atom = atom([]);

// export const setGetNfts1000Atom = atom(
//   (get) => get(nft1000Atom),
//   (get, set, nfts) => {
//     set(nft1000Atom, nfts);
//   },
// );

// export const getAllNftsAtom = atom((get) => [
//   ...get(nft1Atom),
//   ...get(nft10Atom),
//   ...get(nft100Atom),
//   ...get(nft1000Atom),
// ]);

// export const emptyAllNftsAtom = atom(null, (get, set) => {
//   set(nft1Atom, []);
//   set(nft10Atom, []);
//   set(nft100Atom, []);
//   set(nft1000Atom, []);
// });

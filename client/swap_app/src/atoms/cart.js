import { atom } from 'jotai';
import { gldNftAtom } from './nfts';

export const cartAtom = atom([]);

export const getCartAtom = atom((get) => get(cartAtom));

export const addCartItem = (nft, cart) => [...cart, nft];

export const addCartItemAtom = atom(null, (get, set, nft) => {
    set(cartAtom, addCartItem(nft, get(cartAtom)));
});

export const removeCartItemById = (nft, cart) => {
    const index = cart.findIndex((e) => e.name === nft.name);
    if (index !== -1) {
        cart.splice(index, 1);
    }
    return cart.filter((e) => typeof e === 'object');
};

export const removeCartItemByIdAtom = atom(null, (get, set, id) => {
    set(cartAtom, removeCartItemById(id, get(cartAtom)));
});

export const removeAllCartItems = () => [];

export const removeAllItemsInCartAtom = atom(null, (_get, set) => {
    set(cartAtom, removeAllCartItems());
});

export const addAllItemsAtom = atom(null, (get, set, items) => {
    set(cartAtom, items);
});

export const getTotalCartWeight = (cart) =>
    cart.reduce((ac, e) => {
        return ac + e.weight;
    }, 0);

export const getTotalCartWeightAtom = atom((get) => getTotalCartWeight(...[get(cartAtom)]));

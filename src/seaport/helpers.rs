use std::ops::Add;
use std::str::FromStr;

use substreams::scalar::BigInt;

use crate::pb::seaport::{ItemType, OrderFulfilled, SpentItem};

pub struct NftPayment {
    pub item: SpentItem,
    pub payment: SpentItem,
    pub buyer: String,
    pub seller: String,
}

pub fn extract_nft_payment(order: &OrderFulfilled) -> Option<NftPayment> {
    if order.offer.len() == 1 {
        // Get the NFT of the order
        let item = order.offer.get(0).unwrap();
        match item.item_type() {
            ItemType::Erc20 | ItemType::Native => {
                // Offer is an ERC20, so it's a bid for an NFT
                let nft = order.consideration.get(0);
                if nft.is_none() {
                    return None;
                }
                let nft = nft.unwrap();
                return Some(NftPayment {
                    item: SpentItem {
                        item_type: nft.item_type,
                        token: nft.token.clone(),
                        identifier: nft.identifier.clone(),
                        amount: nft.amount.clone(),
                    },
                    payment: item.clone(),
                    buyer: order.offerer.clone(),
                    seller: order.recipient.clone(),
                });
            }

            ItemType::Erc721 | ItemType::Erc721WithCriteria => {
                let payment = order.consideration.get(0);
                if payment.is_none() {
                    return None;
                }
                let payment = payment.unwrap().clone();

                let payment =
                    order
                        .consideration
                        .iter()
                        .skip(1)
                        .fold(payment, |mut memo, token| {
                            if !memo.item_type().eq(&ItemType::Erc20)
                                && !memo.item_type().eq(&ItemType::Native)
                            {
                                token.clone()
                            } else if (token.item_type().eq(&ItemType::Erc20)
                                || token.item_type().eq(&ItemType::Native))
                                && memo.token.eq(&token.token)
                            {
                                memo.amount = BigInt::from_str(&memo.amount)
                                    .unwrap()
                                    .add(BigInt::from_str(&token.amount).unwrap())
                                    .to_string();
                                memo
                            } else {
                                memo
                            }
                        });

                if !payment.item_type().eq(&ItemType::Erc20)
                    && !payment.item_type().eq(&ItemType::Native)
                {
                    return None;
                }
                return Some(NftPayment {
                    item: item.clone(),
                    payment: SpentItem {
                        item_type: payment.item_type,
                        token: payment.token,
                        identifier: payment.identifier,
                        amount: payment.amount,
                    },
                    buyer: order.recipient.clone(),
                    seller: order.offerer.clone(),
                });
            }
            _ => return None,
        }
    } else {
        return None;
    }
}

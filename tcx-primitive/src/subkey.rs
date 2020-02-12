use super::Result;

use crate::ecc::{DeterministicPrivateKey, DeterministicPublicKey, KeyError};

use crate::sr25519::{Sr25519PrivateKey, Sr25519PublicKey};
use crate::{Derive, DeriveJunction, FromHex, PrivateKey, PublicKey, Ss58Codec, ToHex};
use regex::Regex;
use sp_core::crypto::Derive as SpDerive;
use sp_core::crypto::Ss58Codec as SubSs58Codec;
use sp_core::{Pair as TraitPair, Public as TraitPublic};
use std::convert::TryInto;

//
//impl SubDeterministicPrivateKey {
//    /// Construct a new master key from a seed value
//    pub fn from_seed(seed: &[u8]) -> Result<Self> {
//        let sk = Sr25519PrivateKey::from_slice(seed)?;
//        Ok(SubDeterministicPrivateKey(sk))
//    }
//}

impl Derive for Sr25519PrivateKey {
    fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        unimplemented!()
    }
    //    fn derive<T: Iterator<Item = DeriveJunction>>(&self, path: T) -> Result<Self> {
    //        // todo: unwrap
    //        let path = path.map(|x| {
    //            // todo: merge DeriveJunction
    //            match x {
    //                DeriveJunction::Soft(_) =>
    //                    sp_core::crypto::DeriveJunction::Hard([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    //                DeriveJunction::Hard(_) =>
    //                    sp_core::crypto::DeriveJunction::Hard([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
    //            }
    //        }).collect::<Vec<sp_core::crypto::DeriveJunction>>();
    //        let path: std::iter::Iterator<Item = sp_core::DeriveJunction> = path.iter();
    //        Ok(Sr25519PrivateKey(self.0.derive(&path, None).unwrap().0))
    //    }

    fn derive_from_path(&self, path: &str) -> Result<Self> {
        let re_junction =
            Regex::new(r"/(/?[^/]+)").expect("constructed from known-good static value; qed");
        let path = re_junction
            .captures_iter(path)
            .map(|f| sp_core::crypto::DeriveJunction::from(&f[1]));

        Ok(Sr25519PrivateKey(self.0.derive(path, None).unwrap().0))
    }
}

impl Derive for Sr25519PublicKey {
    fn derive<Iter: Iterator<Item = DeriveJunction>>(&self, path: Iter) -> Result<Self> {
        unimplemented!()
    }

    fn derive_from_path(&self, path: &str) -> Result<Self> {
        let re_junction =
            Regex::new(r"/(/?[^/]+)").expect("constructed from known-good static value; qed");
        let path = re_junction
            .captures_iter(path)
            .map(|f| sp_core::crypto::DeriveJunction::from(&f[1]));
        // todo: unwrap()
        Ok(Sr25519PublicKey(self.0.derive(path).unwrap()))
    }
}

impl DeterministicPrivateKey for Sr25519PrivateKey {
    type DeterministicPublicKey = Sr25519PublicKey;
    type PrivateKey = Sr25519PrivateKey;

    fn from_seed(seed: &[u8]) -> Result<Self> {
        Sr25519PrivateKey::from_slice(seed)
    }

    fn from_mnemonic(mnemonic: &str) -> Result<Self> {
        Sr25519PrivateKey::from_mnemonic(mnemonic)
    }

    fn private_key(&self) -> Self::PrivateKey {
        self.clone()
    }

    fn deterministic_public_key(&self) -> Self::DeterministicPublicKey {
        Sr25519PublicKey(self.0.public())
    }
}

impl DeterministicPublicKey for Sr25519PublicKey {
    type PublicKey = Sr25519PublicKey;

    fn public_key(&self) -> Self::PublicKey {
        Sr25519PublicKey::from(self.0)
    }
}
//
//impl ToString for Sr25519PrivateKey {
//    fn to_string(&self) -> String {
//
//    }
//}
//
//impl ToString for Sr25519PublicKey {
//    fn to_string(&self) -> String {
//        self.0.to_string()
//    }
//}

impl ToHex for Sr25519PublicKey {
    fn to_hex(&self) -> String {
        //        let mut ret = [0; 74];
        //        let extended_key = self.0;
        //        ret[0] = extended_key.depth as u8;
        //        ret[1..5].copy_from_slice(&extended_key.parent_fingerprint[..]);
        //
        //        BigEndian::write_u32(&mut ret[5..9], u32::from(extended_key.child_number));
        //
        //        ret[9..41].copy_from_slice(&extended_key.chain_code[..]);
        //        ret[41..74].copy_from_slice(&extended_key.public_key.key.serialize()[..]);
        //        hex::encode(ret.to_vec())
        hex::encode(self.0.to_raw_vec())
    }
}

impl FromHex for Sr25519PublicKey {
    fn from_hex(hex: &str) -> Result<Self> {
        //        let data = hex::decode(hex)?;
        //
        //        if data.len() != 74 {
        //            return Err(KeyError::InvalidBase58.into());
        //        }
        //        let cn_int: u32 = BigEndian::read_u32(&data[5..9]);
        //        let child_number: ChildNumber = ChildNumber::from(cn_int);
        //
        //        let epk = ExtendedPubKey {
        //            network: Network::Bitcoin,
        //            depth: data[0],
        //            parent_fingerprint: Fingerprint::from(&data[1..5]),
        //            child_number,
        //            chain_code: ChainCode::from(&data[9..41]),
        //            public_key: PublicKey::from_slice(&data[41..74])
        //                .map_err(|e| base58::Error::Other(e.to_string()))?,
        //        };
        //        Ok(Bip32DeterministicPublicKey(epk))
        let bytes = hex::decode(hex)?;
        let pk = Sr25519PublicKey::from_slice(bytes.as_slice())?;
        Ok(pk)
    }
}
//
//impl Ss58Codec for Bip32DeterministicPublicKey {
//    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)> {
//        let data = base58::from_check(s)?;
//
//        if data.len() != 78 {
//            return Err(KeyError::InvalidBase58.into());
//        }
//        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
//        let child_number: ChildNumber = ChildNumber::from(cn_int);
//
//        let epk = ExtendedPubKey {
//            network: Network::Bitcoin,
//            depth: data[4],
//            parent_fingerprint: Fingerprint::from(&data[5..9]),
//            child_number,
//            chain_code: ChainCode::from(&data[13..45]),
//            public_key: PublicKey::from_slice(&data[45..78])
//                .map_err(|e| base58::Error::Other(e.to_string()))?,
//        };
//
//        let mut network = [0; 4];
//        network.copy_from_slice(&data[0..4]);
//        Ok((Bip32DeterministicPublicKey(epk), network.to_vec()))
//    }
//
//    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
//        let mut ret = [0; 78];
//        let extended_key = self.0;
//        ret[0..4].copy_from_slice(&version[..]);
//        ret[4] = extended_key.depth as u8;
//        ret[5..9].copy_from_slice(&extended_key.parent_fingerprint[..]);
//
//        BigEndian::write_u32(&mut ret[9..13], u32::from(extended_key.child_number));
//
//        ret[13..45].copy_from_slice(&extended_key.chain_code[..]);
//        ret[45..78].copy_from_slice(&extended_key.public_key.key.serialize()[..]);
//        base58::check_encode_slice(&ret[..])
//    }
//}
//
//impl Ss58Codec for Bip32DeterministicPrivateKey {
//    fn from_ss58check_with_version(s: &str) -> Result<(Self, Vec<u8>)> {
//        let data = base58::from_check(s)?;
//
//        if data.len() != 78 {
//            return Err(InvalidLength(data.len()).into());
//        }
//
//        let cn_int: u32 = BigEndian::read_u32(&data[9..13]);
//        let child_number: ChildNumber = ChildNumber::from(cn_int);
//
//        let network = Network::Bitcoin;
//        let epk = ExtendedPrivKey {
//            network,
//            depth: data[4],
//            parent_fingerprint: Fingerprint::from(&data[5..9]),
//            child_number,
//            chain_code: ChainCode::from(&data[13..45]),
//            private_key: bitcoin::PrivateKey {
//                compressed: true,
//                network,
//                key: secp256k1::SecretKey::from_slice(&data[46..78])
//                    .map_err(|e| base58::Error::Other(e.to_string()))?,
//            },
//        };
//        let mut network = [0; 4];
//        network.copy_from_slice(&data[0..4]);
//        Ok((Bip32DeterministicPrivateKey(epk), network.to_vec()))
//    }
//
//    fn to_ss58check_with_version(&self, version: &[u8]) -> String {
//        let mut ret = [0; 78];
//        let extended_key = &self.0;
//
//        ret[0..4].copy_from_slice(&version[..]);
//        ret[4] = extended_key.depth as u8;
//        ret[5..9].copy_from_slice(&extended_key.parent_fingerprint[..]);
//
//        BigEndian::write_u32(&mut ret[9..13], u32::from(extended_key.child_number));
//
//        ret[13..45].copy_from_slice(&extended_key.chain_code[..]);
//        ret[45] = 0;
//        ret[46..78].copy_from_slice(&extended_key.private_key[..]);
//        base58::check_encode_slice(&ret[..])
//    }
//}

#[cfg(test)]
mod tests {

    use super::{Sr25519PrivateKey, Ss58Codec};

    use crate::PrivateKey;

    use bitcoin_hashes::hex::ToHex;
    use bitcoin_hashes::Hash;

    use schnorrkel::{ExpansionMode, MiniSecretKey};
    use sp_core::sr25519::Pair;
    use tcx_constants::coin_info::coin_info_from_param;

    //    #[test]
    //    fn construct_test() {
    //        //       let entropy = "54101bfe06f6fc404289b973d6e4e7cf";
    //        let seed_hex = "9dd32e5182f147ffe08fee7c1b449647b5e17a89d35622c9d603c41b6a3937c717f8cf9db7d5293de58d14680ec3e7b897398026352b84e224017f5b82acc6fa";
    //        let seed = hex::decode(seed_hex).unwrap();
    //        let mini_key =
    //            MiniSecretKey::from_bytes(&seed[..32]).expect("Length is always correct; qed");
    //
    //        //
    //        let kp = mini_key.expand_to_keypair(ExpansionMode::Ed25519);
    //        let pair = Sr25519PrivateKey(Pair::from(kp));
    //        let sub_seed = mini_key.to_bytes();
    //
    //        //       let pair = Pair::from_entropy(&hex::decode("54101bfe06f6fc404289b973d6e4e7cf").unwrap(), None);
    //        assert_eq!(hex::encode(sub_seed), "");
    //    }
}

// This file is part of Gear.

// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Make the set of bag thresholds to be used with pallet-bags-list.

use clap::Parser;
use codec::Encode;
use sp_core::{
    crypto::{Ss58AddressFormat, Ss58Codec, UncheckedFrom},
    H256,
};
use sp_io::hashing::blake2_256;
use sp_runtime::AccountId32;

mod parse;

const VARA_SS58_PREFIX: u16 = 137;

#[derive(Debug, Parser)]
struct Opt {
    /// Unique prefix to ensure we don't occasionally create account with a private key
    #[clap(long, ignore_case = false, default_value = "dmmy//mltsig//id", value_parser = parse::prefix)]
    prefix: String,

    /// Number of samples
    #[clap(long, short, ignore_case = false, default_value = "10")]
    n: u16,
}

fn main() {
    env_logger::init();

    let Opt { prefix, n } = Opt::parse();

    for index in 0..n {
        assert!(prefix.len() <= 16);

        let entropy = (prefix.clone(), index).using_encoded(blake2_256);

        let account_id_32 = AccountId32::unchecked_from(H256::from_slice(&entropy[..]));
        let account_id =
            account_id_32.to_ss58check_with_version(Ss58AddressFormat::custom(VARA_SS58_PREFIX));
        println!("( {account_id}, 0x{} )", hex::encode(account_id_32.clone()));
    }
}

use rocket::serde::json::{ Value, json};
use bip0039::{Count, Mnemonic}; 
use sp_core::{
	hexdisplay::HexDisplay,
    crypto::{Ss58Codec,Ss58AddressFormatRegistry},
};

use sp_runtime::{MultiSigner, traits::IdentifyAccount};

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build() .mount("/", routes![index])
    // 地址
    .mount("/", routes![new_address]) 
}

#[get("/address/new")]
pub fn new_address() -> Value {
    generate::<sp_core::sr25519::Pair>()
}

fn generate<Pair>() -> Value 
    where Pair: sp_core::Pair, Pair::Public: Into<MultiSigner> {
    // 生成随机密码
    // let pg = PasswordGenerator {
    //     length: 8, // 密码长度
    //     numbers: true, // 是否包含数字
    //     lowercase_letters: true, // 是否包含小写
    //     uppercase_letters: true, // 是否包含大写
    //     symbols: false, // 是否包含特殊字符
    //     spaces: false, // 是否包含空格
    //     exclude_similar_characters: false, // 排除类似字符
    //     strict: true, // 严格模式
    // };

    // let pwd = pg.generate_one().unwrap();

    // Generates an English mnemonic with 12 words randomly
  //  let mnemonic = <Mnemonic<English>>::generate(Count::Words12);
    // Or use the default generic type (English) of struct Mnemonic.
    let mnemonic = <Mnemonic>::generate(Count::Words12);
    // Gets the phrase
    let phrase = mnemonic.phrase();

    // 生成地址及助记词
    match Pair::from_phrase(phrase, None) {
        Ok((pair, seed)) => {
            let account = pair.public().into().into_account();
            json!({
                "code": 0,
                "data": {
                    "phrase": phrase,
                    "seed": format!("0x{}", HexDisplay::from(&seed.as_ref())),
                    "public": format!("0x{}", HexDisplay::from(&pair.public().as_ref())),
                    "ss58Address": account.to_ss58check(),
                    "addresses": 
                        {
                            // Acala
                            "acaAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::AcalaAccount.into()),
                            // Bifrost
                            "bncAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::BifrostAccount.into()),
                            // Crust
                            //"cruAddress": account.to_ss58check_with_version(Ss58AddressFormat::Custom(45)),
                            // ChainX, chainx用的是ed25519
                            //"pcxAddress": account.to_ss58check_with_version(Ss58AddressFormat::ChainXAccount),
                            // Polkadot
                            "dotAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::PolkadotAccount.into()),
                            // Kusama
                            "ksmAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::KusamaAccount.into()),
                            // Edgeware
                            "edgAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::EdgewareAccount.into()),
                            // Darwinia
                            "cringAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::DarwiniaAccount.into()),
                            // Kulupu
                            "klpAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::KulupuAccount.into()),
                            // Stafi
                            "fisAddress": account.to_ss58check_with_version(Ss58AddressFormatRegistry::StafiAccount.into()),
                        },
                },
                "msg": "success"
            })
        }
        Err(err) => {
            json!({
                "code": 500,
                "data": {},
                "msg": format!("{:?}", err),
            })
        }
    }
} 

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!, rust web"
}
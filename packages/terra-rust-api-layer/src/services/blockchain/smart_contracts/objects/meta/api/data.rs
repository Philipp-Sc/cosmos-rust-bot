pub mod terra_contracts;
pub mod endpoints; 

use serde::Deserialize;
use serde::Serialize;
 

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GasPrices {
    pub uluna: String,
    pub uusd: String,
    pub usdr: String,
    pub ukrw: String,
    pub umnt: String,
    pub ueur: String,
    pub ucny: String,
    pub ujpy: String,
    pub ugbp: String,
    pub ucad: String,
    pub uchf: String,
    pub uaud: String,
    pub usgd: String,
    pub uthb: String,
    pub usek: String,
    pub unok: String,
    pub udkk: String,
    pub uidr: String,
    pub uphp: String,
    pub uhkd: String, 
}
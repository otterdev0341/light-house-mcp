use serde::{Deserialize, Serialize};


// auth

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ResMeDto {
    pub id: String,
    pub gender: String,
    pub user_role: String,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String
}

// feature
#[derive(Serialize, Deserialize)]
pub struct ResEntryAssetDto{
    pub id: String,
    pub name: String,
    pub asset_type: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListAssetDto {
    pub length: i32,
    pub data: Vec<ResEntryAssetDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryContactDto {
    pub id: String,
    pub name: String,
    pub business_name: String,
    pub phone: String,
    pub description: String,
    pub contact_type_name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListContactDto {
    pub length: i32,
    pub data: Vec<ResEntryContactDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryExpenseDto {
    pub id: String,
    pub description: String,
    pub expense_type_name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListExpenseDto {
    pub length: i32,
    pub data: Vec<ResEntryExpenseDto>,
}


// type
#[derive(Serialize, Deserialize)]
pub struct ResEntryAssetTypeDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListAssestTypeDto {
    pub length: i32,
    pub data: Vec<ResEntryAssetTypeDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryContactTypeDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListContactTypeDto {
    pub length: i32,
    pub data: Vec<ResEntryContactTypeDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryExpenseTypeDto {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListExpenseTypeDto {
    pub length: i32,
    pub data: Vec<ResEntryExpenseTypeDto>,
}



// transaction
#[derive(Serialize, Deserialize)]
pub struct ResEntryPaymentDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub expense_name: String,
    pub contact_name: String,
    pub asset_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListPaymentDto{
    pub length: i32,
    pub data: Vec<ResEntryPaymentDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryIncomeDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub asset_name: String,
    pub contact_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListIncomeDto{
    pub length: i32,
    pub data: Vec<ResEntryIncomeDto>,
}

#[derive(Serialize, Deserialize)]
pub struct ResEntryTransferDto{
    pub id: String,
    pub transaction_type_name: String,
    pub amount: f64,
    pub asset_name: String,
    pub destination_asset_name: String,
    pub contact_name: String,
    pub note: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResListTransferDto{
    pub length: i64,
    pub data: Vec<ResEntryTransferDto>,
}
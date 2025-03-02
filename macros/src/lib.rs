use proc_macro::TokenStream;

mod service_executor;
mod derive_custom_model;

/// A procedural macro to generate an executor function for service methods that use a `self.db_pool` parameter.
/// 
/// This macro will create an executor function that takes a mutable reference to a `SqliteConnection` as a parameter,
/// allowing the connection to be passed in parameters instead of being taken from the `db_pool`. This enables calling
/// executors from other services to ensure transactionality, while controllers can call the normal method without
/// knowing anything about the database.
/// 
/// A tx parameter representing a db connection or transaction is implicitly added, so it will be available in the whole
/// method body. You must assume tx is a '&mut SqliteConnection'.
///
/// # Example
///
/// ```rust
/// #[executor]
/// pub async fn my_service_method(&self, param1: Type1, param2: Type2) -> Result<Type3, Error> {
///     // original method implementation
/// }
/// ```
///
/// The above code will generate:
///
/// ```rust
/// pub async fn my_service_method(&self, param1: Type1, param2: Type2) -> Result<Type3, Error> {
///     {
///         let mut tx = self.db_pool.acquire().await.unwrap();
///         self.my_service_method_executor(&mut tx, param1, param2).await
///     }
/// }
///
/// pub async fn my_service_method_executor(&self, tx: &mut SqliteConnection, param1: Type1, param2: Type2) -> Result<Type3, Error> {
///     // original method implementation
/// }
/// ```
///
/// The `my_service_method_executor` function can be called from other services to ensure transactionality,
/// while `my_service_method` can be called from controllers without exposing database details.
#[proc_macro_attribute]
pub fn executor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    service_executor::executor_impl(_attr, item)
}

#[proc_macro_derive(DeriveCustomModel, attributes(custom_model))]
pub fn derive_custom_model(input: TokenStream) -> TokenStream {
    derive_custom_model::derive_custom_model_impl(input)
}
pub extern crate candid;
pub extern crate ic_cdk;
pub extern crate types;

#[macro_export]
macro_rules! generate_update_call {
    ($method_name:ident) => {
        pub async fn $method_name(
            agent: &ic_agent::Agent,
            canister_id: &candid::Principal,
            args: &$method_name::Args,
        ) -> Result<$method_name::Response, Box<dyn std::error::Error + Sync + std::marker::Send>> {
            use candid::{Decode, Encode};

            let candid_args = Encode!(args)?;

            let method_name = stringify!($method_name);
            let response = agent
                .update(canister_id, method_name)
                .with_arg(candid_args)
                .call_and_wait()
                .await?;

            let result = Decode!(response.as_slice(), $method_name::Response)?;
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! generate_query_call {
    ($method_name:ident) => {
        pub async fn $method_name(
            agent: &ic_agent::Agent,
            canister_id: &candid::Principal,
            args: &$method_name::Args,
        ) -> Result<
            $method_name::Response,
            Box<dyn std::error::Error + std::marker::Send + std::marker::Sync>,
        > {
            use candid::{Decode, Encode};

            let candid_args = Encode!(args)?;

            let method_name = stringify!($method_name);
            let response = agent
                .query(canister_id, method_name)
                .with_arg(candid_args)
                .call()
                .await?;

            Ok(Decode!(response.as_slice(), $method_name::Response)?)
        }
    };
}

#[macro_export]
macro_rules! generate_c2c_call {
    ($method_name:ident) => {
        pub async fn $method_name(
            canister_id: types::CanisterId,
            args: &$method_name::Args,
        ) -> ic_cdk::api::call::CallResult<$method_name::Response> {
            let method_name = concat!(stringify!($method_name), "_msgpack");

            canister_client::make_c2c_call(
                canister_id,
                method_name,
                args,
                msgpack::serialize,
                |r| msgpack::deserialize(r),
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! generate_candid_c2c_call {
    ($method_name:ident) => {
        generate_candid_c2c_call!($method_name, $method_name);
    };
    ($method_name:ident, $external_canister_method_name:ident) => {
        pub async fn $method_name<A>(
            canister_id: $crate::types::CanisterId,
            args: A,
        ) -> $crate::ic_cdk::api::call::CallResult<$method_name::Response>
        where
            A: std::borrow::Borrow<$method_name::Args>,
        {
            let method_name = stringify!($external_canister_method_name);

            ::canister_client::make_c2c_call(
                canister_id,
                method_name,
                args.borrow(),
                $crate::candid::encode_one,
                |r| $crate::candid::decode_one(r),
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! generate_candid_c2c_call_with_payment {
    ($method_name:ident) => {
        pub async fn $method_name(
            canister_id: ::types::CanisterId,
            args: &$method_name::Args,
            cycles: ::types::Cycles,
        ) -> ::ic_cdk::api::call::CallResult<$method_name::Response> {
            let method_name = stringify!($method_name);

            canister_client::make_c2c_call_with_payment(
                canister_id,
                method_name,
                args,
                ::candid::encode_one,
                |r| ::candid::decode_one(r),
                cycles,
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! generate_candid_c2c_call_tuple_args {
    ($method_name:ident) => {
        ::canister_client::generate_candid_c2c_call_tuple_args!($method_name, $method_name);
    };
    ($method_name:ident, $external_canister_method_name:ident) => {
        pub async fn $method_name(
            canister_id: ::types::CanisterId,
            args: $method_name::Args,
        ) -> ::ic_cdk::api::call::CallResult<$method_name::Response> {
            let method_name = stringify!($external_canister_method_name);

            canister_client::make_c2c_call(
                canister_id,
                method_name,
                args,
                ::candid::encode_args,
                |r| ::candid::decode_args(r),
            )
            .await
        }
    };
}

#[macro_export]
macro_rules! generate_candid_c2c_call_no_args {
    ($method_name:ident) => {
        ::canister_client::generate_candid_c2c_call_no_args!($method_name, $method_name);
    };
    ($method_name:ident, $external_canister_method_name:ident) => {
        pub async fn $method_name(
            canister_id: $crate::types::CanisterId,
        ) -> $crate::ic_cdk::api::call::CallResult<$method_name::Response> {
            let method_name = stringify!($external_canister_method_name);

            canister_client::make_c2c_call(
                canister_id,
                method_name,
                (),
                $crate::candid::encode_one,
                |r| $crate::candid::decode_one(r),
            )
            .await
        }
    };
}

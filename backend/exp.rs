pub mod errors {
    pub mod errors {
        use serde_derive::Serialize;
        use rej_derive::derive_rej;
        use warp::{
            reject::{self, Reject},
            Rejection, Reply,
        };
        pub struct AuthError {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for AuthError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    AuthError {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "AuthError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for AuthError {}
        impl AuthError {
            pub fn rej() -> Rejection {
                reject::custom(AuthError {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct LoginError {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for LoginError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    LoginError {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "LoginError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for LoginError {}
        impl LoginError {
            pub fn rej() -> Rejection {
                reject::custom(LoginError {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct DataBaseError {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for DataBaseError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    DataBaseError {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "DataBaseError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for DataBaseError {}
        impl DataBaseError {
            pub fn rej() -> Rejection {
                reject::custom(DataBaseError {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct InternalDataBaseError {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InternalDataBaseError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InternalDataBaseError {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InternalDataBaseError");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for InternalDataBaseError {}
        impl InternalDataBaseError {
            pub fn rej() -> Rejection {
                reject::custom(InternalDataBaseError {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct InvalidRequest {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InvalidRequest {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InvalidRequest {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InvalidRequest");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for InvalidRequest {}
        impl InvalidRequest {
            pub fn rej() -> Rejection {
                reject::custom(InvalidRequest {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct InvalidUserDataFormat {
            pub rejection: String,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for InvalidUserDataFormat {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    InvalidUserDataFormat {
                        rejection: ref __self_0_0,
                    } => {
                        let debug_trait_builder =
                            &mut ::core::fmt::Formatter::debug_struct(f, "InvalidUserDataFormat");
                        let _ = ::core::fmt::DebugStruct::field(
                            debug_trait_builder,
                            "rejection",
                            &&(*__self_0_0),
                        );
                        ::core::fmt::DebugStruct::finish(debug_trait_builder)
                    }
                }
            }
        }
        impl Reject for InvalidUserDataFormat {}
        impl InvalidUserDataFormat {
            pub fn rej() -> Rejection {
                reject::custom(InvalidUserDataFormat {
                    rejection: "\"ERR\"".to_owned(),
                })
            }
        }
        pub struct ErrorModel {
            pub message: String,
            pub code: u16,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for ErrorModel {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "ErrorModel",
                        false as usize + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "message",
                        &self.message,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "code",
                        &self.code,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        impl Reply for ErrorModel {
            fn into_response(self) -> warp::reply::Response {
                warp::reply::json(&self).into_response()
            }
        }
    }
}

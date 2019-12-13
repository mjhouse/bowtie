use failure;
use std::error;
use std::fmt;

#[derive(Debug,Fail)]
pub enum BowtieError {
    #[fail(display = "Record not found")]
    RecordNotFound,

    #[fail(display = "Model does not have an id")]
    NoId,

    #[fail(display = "Model failed to create token")]
    TokenCreationFailed,

    #[fail(display = "User token could not be signed")]
    FailedToSign,

    #[fail(display = "User token could not be parsed")]
    FailedToParse,

    #[fail(display = "User token could not be verified")]
    TokenNotVerified,

    #[fail(display = "User cookie could not be decoded")]
    BadCookieFound,

    #[fail(display = "User cookie not found")]
    NoCookieFound
}
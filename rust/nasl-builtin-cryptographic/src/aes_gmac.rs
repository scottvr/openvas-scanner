// SPDX-FileCopyrightText: 2023 Greenbone AG
//
// SPDX-License-Identifier: GPL-2.0-or-later WITH x11vnc-openssl-exception

use nasl_builtin_utils::function_set;

/// NASL function to calculate GMAC with AES128.
///
/// This function expects 3 named arguments key, data and iv either in a string or data type.
#[cfg(feature = "nasl-c-lib")]
fn aes_gmac(
    register: &nasl_builtin_utils::Register,
    _: &nasl_builtin_utils::Context,
) -> Result<nasl_syntax::NaslValue, nasl_builtin_utils::FunctionErrorKind> {
    use crate::{get_data, get_iv, get_key};
    use nasl_c_lib::cryptographic::mac::aes_gmac;

    let key = get_key(register)?;
    let data = get_data(register)?;
    let iv = get_iv(register)?;

    match aes_gmac(data, key, iv) {
        Ok(val) => Ok(val.into()),
        Err(code) => Err(nasl_builtin_utils::FunctionErrorKind::GeneralError(
            nasl_builtin_utils::error::GeneralErrorType::UnexpectedData(format!(
                "Error code {}",
                code
            )),
        )),
    }
}

pub struct AesGmac;

#[cfg(feature = "nasl-c-lib")]
function_set! {
    AesGmac,
    sync_stateless,
    (
        aes_gmac
    )
}

#[cfg(not(feature = "nasl-c-lib"))]
function_set! {
    AesGmac,
    sync_stateless,
    (
    )
}

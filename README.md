# I18nError for Rust

This library provides a convenient way to define and manage error messages with internationalization (i18n) support in Rust.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
i18n_error = "0.1"
rust-i18n = "3"
```

Then, run cargo build to install the dependencies.

## Example

```rust
use i18n_error::{I18nError, LanguageCode, ToI18nString};

#[macro_use]
extern crate rust_i18n;

rust_i18n::i18n!("locales");

#[derive(I18nError)]
#[i18n_language_codes(En, Fr)]
enum UseCaseError {
    #[i18n_key("error.UseCaseError.AuthorizationError")]
    Authorization,
    #[i18n_delegate]
    Domain(DomainError),
}


#[derive(I18nError)]
#[i18n_language_codes(En, Fr)]
enum DomainError {
    #[i18n_key("error.DomainError.ResourceNotFound")]
    ResourceNotFound,
}

fn main() {
    let error = UseCaseError::Authorization;
    assert_eq!(error.to_i18n_string(LanguageCode::En), "You do not have permission.".to_string());
    assert_eq!(error.to_i18n_string(LanguageCode::Fr), "Vous n'avez pas la permission.".to_string());

    let error = UseCaseError::Domain(DomainError::ResourceNotFound);
    assert_eq!(error.to_i18n_string(LanguageCode::En), "Resource not found.".to_string());
    assert_eq!(error.to_i18n_string(LanguageCode::Fr), "Ressource non trouvée.".to_string());
}
```

1. i18n Preparation
   1. Create locales files in `/locales` directory. Refer to the [rust-i18n](https://github.com/longbridgeapp/rust-i18n) for more details.
   2. Load `rust-i18n` in `lib.rs` or `main.rs` file.
2. Define the Error Struct or Enum
   1. Use `#[derive(I18nError)]` to derive the necessary traits.
3. Set the Language Codes
   1. Use `#[i18n_language_codes(En, Fr)]`. to specify the supported languages.
   2. `LanguageCode` based on ISO 639-1. Refer to the [rust-i18n/LanguageCode](https://github.com/poi2/i18n_error/blob/main/src/language_code.rs#L9) for more details.
4. Define the Error Variants
   1. Use ``#[i18n_key("{key of message in locale file}")]` to map error variants to messages in the locale files.
   2. This allows error messages to be generated from the locale file.
5. Delegating Error Message Generation
   1. If you want to delegate the generation of error messages to an inner error, use `#[i18n_delegate]`.
   2. The inner error must also be defined with `#[derive(I18nError)]`.
   3. This allows the error message to be generated from the inner error.

## Details

### Preparing Locale Files

`I18nError` relies on `rust-i18n`. You need to prepare your locale files in a format that conforms to `rust-i18n` standards. For more details, refer to the [rust-i18n](https://github.com/longbridgeapp/rust-i18n) documentation.

In `I18nError`, locale files must be named using ISO 639-1 codes in lowercase. To support English and French, you need to create `/locales/en.toml` and `/locales/fr.toml`.

### Generating Error Messages

The string specified with `i18n_key` corresponds to a YAML path in the locale file. For instance, if the key is `error.UseCaseError.AuthorizationError`, the structure of the locale file should be as follows:

```yaml
"error":
  "UseCaseError":
    "AuthorizationError": "You do not have permission."
```

If the corresponding key does not exist in the locale file, the string `{LanguageCode in lowercase}.{key}` will be returned.

### Delegating Message Generation

If an enum variant contains an error object, you can delegate the message generation to the inner error object by using the `#[i18n_delegate]` attribute. For example, in the `UseCaseError::Domain` variant, the message generation is delegated to `DomainError` because i18n_delegate is specified.

```rust
#[derive(I18nError)]
#[i18n_language_codes(En, Fr)]
enum UseCaseError {
    #[i18n_delegate]
    Domain(DomainError),
}
```

In this case, `DomainError` must derive `I18nError`.

```rust
#[derive(I18nError)]
#[i18n_language_codes(En, Fr)]
enum DomainError {
    #[i18n_key("error.DomainError.ResourceNotFound")]
    ResourceNotFound,
}
```

If `DomainError` does not derive `I18nError`, a compile error will occur.

```rust
enum DomainError {
    ResourceNotFound,
}

error[E0599]: no method named `to_i18n_string` found for reference `&DomainError` in the current scope
  --> src/lib.rs:28:14
   |
28 |     #[derive(I18nError)]
   |              ^^^^^^^^^ method not found in `&DomainError`
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `to_i18n_string`, perhaps you need to implement it:
           candidate #1: `ToI18nString`
   = note: this error originates in the derive macro `I18nError` (in Nightly builds, run with -Z macro-backtrace for more info)

rustcE0599
```

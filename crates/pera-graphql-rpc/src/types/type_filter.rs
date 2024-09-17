// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: BSD-3-Clause-Clear

use super::{pera_address::PeraAddress, string_input::impl_string_input};
use crate::filter;
use crate::raw_query::RawQuery;
use async_graphql::*;
use move_core_types::language_storage::StructTag;
use pera_types::{
    parse_pera_address, parse_pera_fq_name, parse_pera_module_id, parse_pera_struct_tag,
    parse_pera_type_tag, TypeTag,
};
use std::{fmt, result::Result, str::FromStr};

/// A GraphQL scalar containing a filter on types that requires an exact match.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExactTypeFilter(pub TypeTag);

/// GraphQL scalar containing a filter on types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TypeFilter {
    /// Filter the type by the package or module it's from.
    ByModule(ModuleFilter),

    /// If the struct tag has type parameters, treat it as an exact filter on that instantiation,
    /// otherwise treat it as either a filter on all generic instantiations of the type, or an exact
    /// match on the type with no type parameters. E.g.
    ///
    ///  0x2::coin::Coin
    ///
    /// would match both 0x2::coin::Coin and 0x2::coin::Coin<0x2::pera::PERA>.
    ByType(StructTag),
}

/// GraphQL scalar containing a filter on fully-qualified names.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FqNameFilter {
    /// Filter the module member by the package or module it's from.
    ByModule(ModuleFilter),

    /// Exact match on the module member.
    ByFqName(PeraAddress, String, String),
}

/// GraphQL scalar containing a filter on modules.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ModuleFilter {
    /// Filter the module by the package it's from.
    ByPackage(PeraAddress),

    /// Exact match on the module.
    ByModule(PeraAddress, String),
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error("Invalid filter, expected: {0}")]
    InvalidFormat(&'static str),
}

impl TypeFilter {
    /// Modify `query` to apply this filter to `field`, returning the new query.
    pub(crate) fn apply_raw(
        &self,
        mut query: RawQuery,
        type_field: &str,
        package_field: &str,
        module_field: &str,
        name_field: &str,
    ) -> RawQuery {
        match self {
            TypeFilter::ByModule(ModuleFilter::ByPackage(p)) => {
                let statement = format!(
                    "{} = '\\x{}'::bytea",
                    package_field,
                    hex::encode(p.into_vec())
                );
                query = filter!(query, statement);
            }

            TypeFilter::ByModule(ModuleFilter::ByModule(p, m)) => {
                let statement = format!(
                    "{} = '\\x{}'::bytea",
                    package_field,
                    hex::encode(p.into_vec())
                );
                query = filter!(query, statement);
                let m = m.to_string();
                let statement = module_field.to_string() + " = {}";
                query = filter!(query, statement, m);
            }

            // A type filter without type parameters is interpreted as either an exact match, or a
            // match for all generic instantiations of the type.
            TypeFilter::ByType(tag) if tag.type_params.is_empty() => {
                let m = tag.module.to_string();
                let n = tag.name.to_string();
                let statement = format!(
                    "{} = '\\x{}'::bytea",
                    package_field,
                    hex::encode(tag.address.to_vec())
                );
                query = filter!(query, statement);
                let statement = module_field.to_string() + " = {}";
                query = filter!(query, statement, m);
                let statement = name_field.to_string() + " = {}";
                query = filter!(query, statement, n);
            }

            TypeFilter::ByType(tag) => {
                let m = tag.module.to_string();
                let n = tag.name.to_string();
                let statement = format!(
                    "{} = '\\x{}'::bytea",
                    package_field,
                    hex::encode(tag.address.to_vec())
                );
                query = filter!(query, statement);
                let statement = module_field.to_string() + " = {}";
                query = filter!(query, statement, m);
                let statement = name_field.to_string() + " = {}";
                query = filter!(query, statement, n);
                let exact_pattern = tag.to_canonical_string(/* with_prefix */ true);
                let statement = type_field.to_string() + " = {}";
                query = filter!(query, statement, exact_pattern);
            }
        }

        query
    }

    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module member's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        use ModuleFilter as M;
        use TypeFilter as T;

        match (&self, &other) {
            (T::ByModule(m), T::ByModule(n)) => m.clone().intersect(n.clone()).map(T::ByModule),

            (T::ByType(s), T::ByType(t)) if s.type_params.is_empty() => {
                ((&s.address, &s.module, &s.name) == (&t.address, &t.module, &t.name))
                    .then_some(other)
            }

            (T::ByType(s), T::ByType(t)) if t.type_params.is_empty() => {
                ((&s.address, &s.module, &s.name) == (&t.address, &t.module, &t.name))
                    .then_some(self)
            }

            // If both sides are type filters, then at this point, we know that if they are both
            // struct tags, neither has empty type parameters so we can treat both filters as exact
            // type queries which must be equal to each other to intersect.
            (T::ByType(_), T::ByType(_)) => (self == other).then_some(self),

            (T::ByType(s), T::ByModule(M::ByPackage(q))) => {
                (PeraAddress::from(s.address) == *q).then_some(self)
            }

            (T::ByType(s), T::ByModule(M::ByModule(q, n))) => {
                ((PeraAddress::from(s.address), s.module.as_str()) == (*q, n.as_str()))
                    .then_some(self)
            }

            (T::ByModule(M::ByPackage(p)), T::ByType(t)) => {
                (PeraAddress::from(t.address) == *p).then_some(other)
            }

            (T::ByModule(M::ByModule(p, m)), T::ByType(t)) => {
                ((PeraAddress::from(t.address), t.module.as_str()) == (*p, m.as_str()))
                    .then_some(other)
            }
        }
    }
}

impl FqNameFilter {
    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module member's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        use FqNameFilter as F;
        use ModuleFilter as M;

        match (&self, &other) {
            (F::ByModule(m), F::ByModule(n)) => m.clone().intersect(n.clone()).map(F::ByModule),
            (F::ByFqName(_, _, _), F::ByFqName(_, _, _)) => (self == other).then_some(self),

            (F::ByFqName(p, _, _), F::ByModule(M::ByPackage(q))) => (p == q).then_some(self),
            (F::ByModule(M::ByPackage(p)), F::ByFqName(q, _, _)) => (p == q).then_some(other),

            (F::ByFqName(p, m, _), F::ByModule(M::ByModule(q, n))) => {
                ((p, m) == (q, n)).then_some(self)
            }

            (F::ByModule(M::ByModule(p, m)), F::ByFqName(q, n, _)) => {
                ((p, m) == (q, n)).then_some(other)
            }
        }
    }
}

impl ModuleFilter {
    /// Try to create a filter whose results are the intersection of the results of the input
    /// filters (`self` and `other`). This may not be possible if the resulting filter is
    /// inconsistent (e.g. a filter that requires the module's package to be at two different
    /// addresses simultaneously), in which case `None` is returned.
    pub(crate) fn intersect(self, other: Self) -> Option<Self> {
        match (&self, &other) {
            (Self::ByPackage(_), Self::ByPackage(_))
            | (Self::ByModule(_, _), Self::ByModule(_, _)) => (self == other).then_some(self),

            (Self::ByPackage(p), Self::ByModule(q, _)) => (p == q).then_some(other),
            (Self::ByModule(p, _), Self::ByPackage(q)) => (p == q).then_some(self),
        }
    }
}

impl_string_input!(ExactTypeFilter);
impl_string_input!(TypeFilter);
impl_string_input!(FqNameFilter);
impl_string_input!(ModuleFilter);

impl FromStr for ExactTypeFilter {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(tag) = parse_pera_type_tag(s) {
            Ok(ExactTypeFilter(tag))
        } else {
            Err(Error::InvalidFormat(
                "package::module::type<type_params> or primitive type",
            ))
        }
    }
}

impl FromStr for TypeFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(tag) = parse_pera_struct_tag(s) {
            Ok(TypeFilter::ByType(tag))
        } else if let Ok(filter) = ModuleFilter::from_str(s) {
            Ok(TypeFilter::ByModule(filter))
        } else {
            Err(Error::InvalidFormat(
                "package[::module[::type[<type_params>]]] or primitive type",
            ))
        }
    }
}

impl FromStr for FqNameFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok((module, name)) = parse_pera_fq_name(s) {
            Ok(FqNameFilter::ByFqName(
                PeraAddress::from(*module.address()),
                module.name().to_string(),
                name,
            ))
        } else if let Ok(filter) = ModuleFilter::from_str(s) {
            Ok(FqNameFilter::ByModule(filter))
        } else {
            Err(Error::InvalidFormat("package[::module[::function]]"))
        }
    }
}

impl FromStr for ModuleFilter {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        if let Ok(module) = parse_pera_module_id(s) {
            Ok(ModuleFilter::ByModule(
                PeraAddress::from(*module.address()),
                module.name().to_string(),
            ))
        } else if let Ok(package) = parse_pera_address(s) {
            Ok(ModuleFilter::ByPackage(package.into()))
        } else {
            Err(Error::InvalidFormat("package[::module]"))
        }
    }
}

impl fmt::Display for ModuleFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModuleFilter::ByPackage(p) => write!(f, "{p}::"),
            ModuleFilter::ByModule(p, m) => write!(f, "{p}::{m}::"),
        }
    }
}

impl fmt::Display for FqNameFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FqNameFilter::ByModule(m) => write!(f, "{m}"),
            FqNameFilter::ByFqName(p, m, n) => write!(f, "{p}::{m}::{n}"),
        }
    }
}

impl fmt::Display for TypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeFilter::ByModule(m) => write!(f, "{m}"),
            TypeFilter::ByType(t) => {
                write!(f, "{}", t.to_canonical_display(/* with_prefix */ true))
            }
        }
    }
}

impl fmt::Display for ExactTypeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_canonical_display(/* with_prefix */ true))
    }
}

impl From<StructTag> for TypeFilter {
    fn from(tag: StructTag) -> Self {
        TypeFilter::ByType(tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn test_valid_exact_type_filters() {
        let inputs = [
            "u8",
            "address",
            "bool",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::pera::PERA>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedPera>",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| ExactTypeFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            u8
            address
            bool
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::pera::PERA>
            vector<u256>
            vector<0x0000000000000000000000000000000000000000000000000000000000000003::staking_pool::StakedPera>"#]];
        expect.assert_eq(&filters.join("\n"))
    }

    #[test]
    fn test_valid_type_filters() {
        let inputs = [
            "0x2",
            "0x2::coin",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::pera::PERA>",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| TypeFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            0x0000000000000000000000000000000000000000000000000000000000000002::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::Coin<0x0000000000000000000000000000000000000000000000000000000000000002::pera::PERA>"#]];
        expect.assert_eq(&filters.join("\n"))
    }

    #[test]
    fn test_valid_function_filters() {
        let inputs = [
            "0x2",
            "0x2::coin",
            "0x2::object::new",
            "0x2::tx_context::TxContext",
        ]
        .into_iter();

        let filters: Vec<_> = inputs
            .map(|i| FqNameFilter::from_str(i).unwrap().to_string())
            .collect();

        let expect = expect![[r#"
            0x0000000000000000000000000000000000000000000000000000000000000002::
            0x0000000000000000000000000000000000000000000000000000000000000002::coin::
            0x0000000000000000000000000000000000000000000000000000000000000002::object::new
            0x0000000000000000000000000000000000000000000000000000000000000002::tx_context::TxContext"#]];
        expect.assert_eq(&filters.join("\n"));
    }

    #[test]
    fn test_invalid_function_filters() {
        for invalid_function_filter in [
            "0x2::coin::Coin<0x2::pera::PERA>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedPera>",
        ] {
            assert!(FqNameFilter::from_str(invalid_function_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_exact_type_filters() {
        for invalid_exact_type_filter in [
            "not_a_real_type",
            "0x1:missing::colon",
            "0x2",
            "0x2::coin",
            "0x2::trailing::",
            "0x3::mismatched::bra<0x4::ke::ts",
            "vector",
        ] {
            assert!(ExactTypeFilter::from_str(invalid_exact_type_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_type_filters() {
        for invalid_type_filter in [
            "not_a_real_type",
            "0x1:missing::colon",
            "0x2::trailing::",
            "0x3::mismatched::bra<0x4::ke::ts",
            "vector",
        ] {
            assert!(TypeFilter::from_str(invalid_type_filter).is_err());
        }
    }

    #[test]
    fn test_invalid_module_filters() {
        for invalid_module_filter in [
            "u8",
            "address",
            "bool",
            "0x2::coin::Coin",
            "0x2::coin::Coin<0x2::pera::PERA>",
            "vector<u256>",
            "vector<0x3::staking_pool::StakedPera>",
        ] {
            assert!(ModuleFilter::from_str(invalid_module_filter).is_err());
        }
    }

    #[test]
    fn test_fqname_intersection() {
        let pera = FqNameFilter::from_str("0x2").unwrap();
        let coin = FqNameFilter::from_str("0x2::coin").unwrap();
        let take = FqNameFilter::from_str("0x2::coin::take").unwrap();

        let std = FqNameFilter::from_str("0x1").unwrap();
        let string = FqNameFilter::from_str("0x1::string").unwrap();
        let utf8 = FqNameFilter::from_str("0x1::string::utf8").unwrap();

        assert_eq!(pera.clone().intersect(pera.clone()), Some(pera.clone()));
        assert_eq!(pera.clone().intersect(coin.clone()), Some(coin.clone()));
        assert_eq!(pera.clone().intersect(take.clone()), Some(take.clone()));
        assert_eq!(take.clone().intersect(coin.clone()), Some(take.clone()));

        assert_eq!(pera.clone().intersect(std.clone()), None);
        assert_eq!(pera.clone().intersect(string.clone()), None);
        assert_eq!(utf8.clone().intersect(coin.clone()), None);
    }

    #[test]
    fn test_type_intersection() {
        let pera = TypeFilter::from_str("0x2").unwrap();
        let coin_mod = TypeFilter::from_str("0x2::coin").unwrap();
        let coin_typ = TypeFilter::from_str("0x2::coin::Coin").unwrap();
        let coin_pera = TypeFilter::from_str("0x2::coin::Coin<0x2::pera::PERA>").unwrap();
        let coin_usd = TypeFilter::from_str("0x2::coin::Coin<0x3::usd::USD>").unwrap();
        let std_utf8 = TypeFilter::from_str("0x1::string::String").unwrap();

        assert_eq!(
            pera.clone().intersect(coin_mod.clone()),
            Some(coin_mod.clone())
        );

        assert_eq!(
            coin_typ.clone().intersect(coin_mod.clone()),
            Some(coin_typ.clone())
        );

        assert_eq!(
            coin_pera.clone().intersect(coin_typ.clone()),
            Some(coin_pera.clone())
        );

        assert_eq!(pera.clone().intersect(std_utf8.clone()), None);
        assert_eq!(coin_pera.clone().intersect(coin_usd.clone()), None);
        assert_eq!(coin_typ.clone().intersect(std_utf8.clone()), None);
        assert_eq!(coin_pera.clone().intersect(std_utf8.clone()), None);
    }
}

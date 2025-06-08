// @generated
impl serde::Serialize for DeprecationDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.effective_at.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.DeprecationDetails", len)?;
        if !self.effective_at.is_empty() {
            struct_ser.serialize_field("effectiveAt", &self.effective_at)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeprecationDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "effective_at",
            "effectiveAt",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EffectiveAt,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "effectiveAt" | "effective_at" => Ok(GeneratedField::EffectiveAt),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeprecationDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.DeprecationDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeprecationDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut effective_at__ = None;
                let mut description__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EffectiveAt => {
                            if effective_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveAt"));
                            }
                            effective_at__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(DeprecationDetails {
                    effective_at: effective_at__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.DeprecationDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FieldBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "FIELD_BEHAVIOR_UNSPECIFIED",
            Self::Immutable => "IMMUTABLE",
            Self::Identifier => "IDENTIFIER",
            Self::InputOnly => "INPUT_ONLY",
            Self::OutputOnly => "OUTPUT_ONLY",
            Self::MeaningfulEmptyValue => "MEANINGFUL_EMPTY_VALUE",
            Self::NonEmptyDefault => "NON_EMPTY_DEFAULT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for FieldBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FIELD_BEHAVIOR_UNSPECIFIED",
            "IMMUTABLE",
            "IDENTIFIER",
            "INPUT_ONLY",
            "OUTPUT_ONLY",
            "MEANINGFUL_EMPTY_VALUE",
            "NON_EMPTY_DEFAULT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FieldBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FIELD_BEHAVIOR_UNSPECIFIED" => Ok(FieldBehavior::Unspecified),
                    "IMMUTABLE" => Ok(FieldBehavior::Immutable),
                    "IDENTIFIER" => Ok(FieldBehavior::Identifier),
                    "INPUT_ONLY" => Ok(FieldBehavior::InputOnly),
                    "OUTPUT_ONLY" => Ok(FieldBehavior::OutputOnly),
                    "MEANINGFUL_EMPTY_VALUE" => Ok(FieldBehavior::MeaningfulEmptyValue),
                    "NON_EMPTY_DEFAULT" => Ok(FieldBehavior::NonEmptyDefault),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for RegionRouting {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.nid.is_empty() {
            len += 1;
        }
        if self.disabled {
            len += 1;
        }
        if self.strict {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.RegionRouting", len)?;
        if !self.nid.is_empty() {
            struct_ser.serialize_field("nid", &self.nid)?;
        }
        if self.disabled {
            struct_ser.serialize_field("disabled", &self.disabled)?;
        }
        if self.strict {
            struct_ser.serialize_field("strict", &self.strict)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RegionRouting {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "nid",
            "disabled",
            "strict",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Nid,
            Disabled,
            Strict,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "nid" => Ok(GeneratedField::Nid),
                            "disabled" => Ok(GeneratedField::Disabled),
                            "strict" => Ok(GeneratedField::Strict),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RegionRouting;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.RegionRouting")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RegionRouting, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut nid__ = None;
                let mut disabled__ = None;
                let mut strict__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Nid => {
                            if nid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nid"));
                            }
                            nid__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Disabled => {
                            if disabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disabled"));
                            }
                            disabled__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Strict => {
                            if strict__.is_some() {
                                return Err(serde::de::Error::duplicate_field("strict"));
                            }
                            strict__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(RegionRouting {
                    nid: nid__.unwrap_or_default(),
                    disabled: disabled__.unwrap_or_default(),
                    strict: strict__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.RegionRouting", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceBehavior {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "RESOURCE_BEHAVIOR_UNSPECIFIED",
            Self::Movable => "MOVABLE",
            Self::Unnamed => "UNNAMED",
            Self::ImmutableName => "IMMUTABLE_NAME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ResourceBehavior {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "RESOURCE_BEHAVIOR_UNSPECIFIED",
            "MOVABLE",
            "UNNAMED",
            "IMMUTABLE_NAME",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceBehavior;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "RESOURCE_BEHAVIOR_UNSPECIFIED" => Ok(ResourceBehavior::Unspecified),
                    "MOVABLE" => Ok(ResourceBehavior::Movable),
                    "UNNAMED" => Ok(ResourceBehavior::Unnamed),
                    "IMMUTABLE_NAME" => Ok(ResourceBehavior::ImmutableName),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}

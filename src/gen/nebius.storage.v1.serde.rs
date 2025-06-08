// @generated
impl serde::Serialize for Bucket {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        if self.status.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.Bucket", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if let Some(v) = self.status.as_ref() {
            struct_ser.serialize_field("status", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Bucket {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "status",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            Status,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            "status" => Ok(GeneratedField::Status),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Bucket;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.Bucket")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Bucket, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut status__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Bucket {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.Bucket", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BucketCounters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.storage_class != 0 {
            len += 1;
        }
        if self.counters.is_some() {
            len += 1;
        }
        if self.non_current_counters.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.BucketCounters", len)?;
        if self.storage_class != 0 {
            let v = StorageClass::try_from(self.storage_class)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.storage_class)))?;
            struct_ser.serialize_field("storageClass", &v)?;
        }
        if let Some(v) = self.counters.as_ref() {
            struct_ser.serialize_field("counters", v)?;
        }
        if let Some(v) = self.non_current_counters.as_ref() {
            struct_ser.serialize_field("nonCurrentCounters", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BucketCounters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "storage_class",
            "storageClass",
            "counters",
            "non_current_counters",
            "nonCurrentCounters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StorageClass,
            Counters,
            NonCurrentCounters,
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
                            "storageClass" | "storage_class" => Ok(GeneratedField::StorageClass),
                            "counters" => Ok(GeneratedField::Counters),
                            "nonCurrentCounters" | "non_current_counters" => Ok(GeneratedField::NonCurrentCounters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BucketCounters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.BucketCounters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BucketCounters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut storage_class__ = None;
                let mut counters__ = None;
                let mut non_current_counters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::StorageClass => {
                            if storage_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageClass"));
                            }
                            storage_class__ = Some(map_.next_value::<StorageClass>()? as i32);
                        }
                        GeneratedField::Counters => {
                            if counters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counters"));
                            }
                            counters__ = map_.next_value()?;
                        }
                        GeneratedField::NonCurrentCounters => {
                            if non_current_counters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonCurrentCounters"));
                            }
                            non_current_counters__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BucketCounters {
                    storage_class: storage_class__.unwrap_or_default(),
                    counters: counters__,
                    non_current_counters: non_current_counters__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.BucketCounters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BucketSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.versioning_policy != 0 {
            len += 1;
        }
        if self.max_size_bytes != 0 {
            len += 1;
        }
        if self.lifecycle_configuration.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.BucketSpec", len)?;
        if self.versioning_policy != 0 {
            let v = VersioningPolicy::try_from(self.versioning_policy)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.versioning_policy)))?;
            struct_ser.serialize_field("versioningPolicy", &v)?;
        }
        if self.max_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxSizeBytes", ToString::to_string(&self.max_size_bytes).as_str())?;
        }
        if let Some(v) = self.lifecycle_configuration.as_ref() {
            struct_ser.serialize_field("lifecycleConfiguration", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BucketSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "versioning_policy",
            "versioningPolicy",
            "max_size_bytes",
            "maxSizeBytes",
            "lifecycle_configuration",
            "lifecycleConfiguration",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VersioningPolicy,
            MaxSizeBytes,
            LifecycleConfiguration,
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
                            "versioningPolicy" | "versioning_policy" => Ok(GeneratedField::VersioningPolicy),
                            "maxSizeBytes" | "max_size_bytes" => Ok(GeneratedField::MaxSizeBytes),
                            "lifecycleConfiguration" | "lifecycle_configuration" => Ok(GeneratedField::LifecycleConfiguration),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BucketSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.BucketSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BucketSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut versioning_policy__ = None;
                let mut max_size_bytes__ = None;
                let mut lifecycle_configuration__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::VersioningPolicy => {
                            if versioning_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("versioningPolicy"));
                            }
                            versioning_policy__ = Some(map_.next_value::<VersioningPolicy>()? as i32);
                        }
                        GeneratedField::MaxSizeBytes => {
                            if max_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxSizeBytes"));
                            }
                            max_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LifecycleConfiguration => {
                            if lifecycle_configuration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lifecycleConfiguration"));
                            }
                            lifecycle_configuration__ = map_.next_value()?;
                        }
                    }
                }
                Ok(BucketSpec {
                    versioning_policy: versioning_policy__.unwrap_or_default(),
                    max_size_bytes: max_size_bytes__.unwrap_or_default(),
                    lifecycle_configuration: lifecycle_configuration__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.BucketSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BucketStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.counters.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.suspension_state != 0 {
            len += 1;
        }
        if self.deleted_at.is_some() {
            len += 1;
        }
        if self.purge_at.is_some() {
            len += 1;
        }
        if !self.domain_name.is_empty() {
            len += 1;
        }
        if !self.region.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.BucketStatus", len)?;
        if !self.counters.is_empty() {
            struct_ser.serialize_field("counters", &self.counters)?;
        }
        if self.state != 0 {
            let v = bucket_status::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if self.suspension_state != 0 {
            let v = bucket_status::SuspensionState::try_from(self.suspension_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.suspension_state)))?;
            struct_ser.serialize_field("suspensionState", &v)?;
        }
        if let Some(v) = self.deleted_at.as_ref() {
            struct_ser.serialize_field("deletedAt", v)?;
        }
        if let Some(v) = self.purge_at.as_ref() {
            struct_ser.serialize_field("purgeAt", v)?;
        }
        if !self.domain_name.is_empty() {
            struct_ser.serialize_field("domainName", &self.domain_name)?;
        }
        if !self.region.is_empty() {
            struct_ser.serialize_field("region", &self.region)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BucketStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "counters",
            "state",
            "suspension_state",
            "suspensionState",
            "deleted_at",
            "deletedAt",
            "purge_at",
            "purgeAt",
            "domain_name",
            "domainName",
            "region",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Counters,
            State,
            SuspensionState,
            DeletedAt,
            PurgeAt,
            DomainName,
            Region,
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
                            "counters" => Ok(GeneratedField::Counters),
                            "state" => Ok(GeneratedField::State),
                            "suspensionState" | "suspension_state" => Ok(GeneratedField::SuspensionState),
                            "deletedAt" | "deleted_at" => Ok(GeneratedField::DeletedAt),
                            "purgeAt" | "purge_at" => Ok(GeneratedField::PurgeAt),
                            "domainName" | "domain_name" => Ok(GeneratedField::DomainName),
                            "region" => Ok(GeneratedField::Region),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BucketStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.BucketStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BucketStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut counters__ = None;
                let mut state__ = None;
                let mut suspension_state__ = None;
                let mut deleted_at__ = None;
                let mut purge_at__ = None;
                let mut domain_name__ = None;
                let mut region__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Counters => {
                            if counters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("counters"));
                            }
                            counters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<bucket_status::State>()? as i32);
                        }
                        GeneratedField::SuspensionState => {
                            if suspension_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("suspensionState"));
                            }
                            suspension_state__ = Some(map_.next_value::<bucket_status::SuspensionState>()? as i32);
                        }
                        GeneratedField::DeletedAt => {
                            if deleted_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deletedAt"));
                            }
                            deleted_at__ = map_.next_value()?;
                        }
                        GeneratedField::PurgeAt => {
                            if purge_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purgeAt"));
                            }
                            purge_at__ = map_.next_value()?;
                        }
                        GeneratedField::DomainName => {
                            if domain_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("domainName"));
                            }
                            domain_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Region => {
                            if region__.is_some() {
                                return Err(serde::de::Error::duplicate_field("region"));
                            }
                            region__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BucketStatus {
                    counters: counters__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    suspension_state: suspension_state__.unwrap_or_default(),
                    deleted_at: deleted_at__,
                    purge_at: purge_at__,
                    domain_name: domain_name__.unwrap_or_default(),
                    region: region__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.BucketStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bucket_status::State {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATE_UNSPECIFIED",
            Self::Creating => "CREATING",
            Self::Active => "ACTIVE",
            Self::Updating => "UPDATING",
            Self::ScheduledForDeletion => "SCHEDULED_FOR_DELETION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for bucket_status::State {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNSPECIFIED",
            "CREATING",
            "ACTIVE",
            "UPDATING",
            "SCHEDULED_FOR_DELETION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bucket_status::State;

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
                    "STATE_UNSPECIFIED" => Ok(bucket_status::State::Unspecified),
                    "CREATING" => Ok(bucket_status::State::Creating),
                    "ACTIVE" => Ok(bucket_status::State::Active),
                    "UPDATING" => Ok(bucket_status::State::Updating),
                    "SCHEDULED_FOR_DELETION" => Ok(bucket_status::State::ScheduledForDeletion),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for bucket_status::SuspensionState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "SUSPENSION_STATE_UNSPECIFIED",
            Self::NotSuspended => "NOT_SUSPENDED",
            Self::Suspended => "SUSPENDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for bucket_status::SuspensionState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "SUSPENSION_STATE_UNSPECIFIED",
            "NOT_SUSPENDED",
            "SUSPENDED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bucket_status::SuspensionState;

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
                    "SUSPENSION_STATE_UNSPECIFIED" => Ok(bucket_status::SuspensionState::Unspecified),
                    "NOT_SUSPENDED" => Ok(bucket_status::SuspensionState::NotSuspended),
                    "SUSPENDED" => Ok(bucket_status::SuspensionState::Suspended),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.CreateBucketRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CreateBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.CreateBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(CreateBucketRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.CreateBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CurrentBucketCounters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.simple_objects_quantity != 0 {
            len += 1;
        }
        if self.simple_objects_size != 0 {
            len += 1;
        }
        if self.multipart_objects_quantity != 0 {
            len += 1;
        }
        if self.multipart_objects_size != 0 {
            len += 1;
        }
        if self.multipart_uploads_quantity != 0 {
            len += 1;
        }
        if self.inflight_parts_quantity != 0 {
            len += 1;
        }
        if self.inflight_parts_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.CurrentBucketCounters", len)?;
        if self.simple_objects_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("simpleObjectsQuantity", ToString::to_string(&self.simple_objects_quantity).as_str())?;
        }
        if self.simple_objects_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("simpleObjectsSize", ToString::to_string(&self.simple_objects_size).as_str())?;
        }
        if self.multipart_objects_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("multipartObjectsQuantity", ToString::to_string(&self.multipart_objects_quantity).as_str())?;
        }
        if self.multipart_objects_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("multipartObjectsSize", ToString::to_string(&self.multipart_objects_size).as_str())?;
        }
        if self.multipart_uploads_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("multipartUploadsQuantity", ToString::to_string(&self.multipart_uploads_quantity).as_str())?;
        }
        if self.inflight_parts_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inflightPartsQuantity", ToString::to_string(&self.inflight_parts_quantity).as_str())?;
        }
        if self.inflight_parts_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("inflightPartsSize", ToString::to_string(&self.inflight_parts_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CurrentBucketCounters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "simple_objects_quantity",
            "simpleObjectsQuantity",
            "simple_objects_size",
            "simpleObjectsSize",
            "multipart_objects_quantity",
            "multipartObjectsQuantity",
            "multipart_objects_size",
            "multipartObjectsSize",
            "multipart_uploads_quantity",
            "multipartUploadsQuantity",
            "inflight_parts_quantity",
            "inflightPartsQuantity",
            "inflight_parts_size",
            "inflightPartsSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SimpleObjectsQuantity,
            SimpleObjectsSize,
            MultipartObjectsQuantity,
            MultipartObjectsSize,
            MultipartUploadsQuantity,
            InflightPartsQuantity,
            InflightPartsSize,
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
                            "simpleObjectsQuantity" | "simple_objects_quantity" => Ok(GeneratedField::SimpleObjectsQuantity),
                            "simpleObjectsSize" | "simple_objects_size" => Ok(GeneratedField::SimpleObjectsSize),
                            "multipartObjectsQuantity" | "multipart_objects_quantity" => Ok(GeneratedField::MultipartObjectsQuantity),
                            "multipartObjectsSize" | "multipart_objects_size" => Ok(GeneratedField::MultipartObjectsSize),
                            "multipartUploadsQuantity" | "multipart_uploads_quantity" => Ok(GeneratedField::MultipartUploadsQuantity),
                            "inflightPartsQuantity" | "inflight_parts_quantity" => Ok(GeneratedField::InflightPartsQuantity),
                            "inflightPartsSize" | "inflight_parts_size" => Ok(GeneratedField::InflightPartsSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CurrentBucketCounters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.CurrentBucketCounters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CurrentBucketCounters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut simple_objects_quantity__ = None;
                let mut simple_objects_size__ = None;
                let mut multipart_objects_quantity__ = None;
                let mut multipart_objects_size__ = None;
                let mut multipart_uploads_quantity__ = None;
                let mut inflight_parts_quantity__ = None;
                let mut inflight_parts_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SimpleObjectsQuantity => {
                            if simple_objects_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simpleObjectsQuantity"));
                            }
                            simple_objects_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SimpleObjectsSize => {
                            if simple_objects_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simpleObjectsSize"));
                            }
                            simple_objects_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MultipartObjectsQuantity => {
                            if multipart_objects_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multipartObjectsQuantity"));
                            }
                            multipart_objects_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MultipartObjectsSize => {
                            if multipart_objects_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multipartObjectsSize"));
                            }
                            multipart_objects_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MultipartUploadsQuantity => {
                            if multipart_uploads_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multipartUploadsQuantity"));
                            }
                            multipart_uploads_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InflightPartsQuantity => {
                            if inflight_parts_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflightPartsQuantity"));
                            }
                            inflight_parts_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::InflightPartsSize => {
                            if inflight_parts_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("inflightPartsSize"));
                            }
                            inflight_parts_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CurrentBucketCounters {
                    simple_objects_quantity: simple_objects_quantity__.unwrap_or_default(),
                    simple_objects_size: simple_objects_size__.unwrap_or_default(),
                    multipart_objects_quantity: multipart_objects_quantity__.unwrap_or_default(),
                    multipart_objects_size: multipart_objects_size__.unwrap_or_default(),
                    multipart_uploads_quantity: multipart_uploads_quantity__.unwrap_or_default(),
                    inflight_parts_quantity: inflight_parts_quantity__.unwrap_or_default(),
                    inflight_parts_size: inflight_parts_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.CurrentBucketCounters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.purge.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.DeleteBucketRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if let Some(v) = self.purge.as_ref() {
            match v {
                delete_bucket_request::Purge::PurgeAt(v) => {
                    struct_ser.serialize_field("purgeAt", v)?;
                }
                delete_bucket_request::Purge::Ttl(v) => {
                    struct_ser.serialize_field("ttl", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "purge_at",
            "purgeAt",
            "ttl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            PurgeAt,
            Ttl,
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
                            "id" => Ok(GeneratedField::Id),
                            "purgeAt" | "purge_at" => Ok(GeneratedField::PurgeAt),
                            "ttl" => Ok(GeneratedField::Ttl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.DeleteBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut purge__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PurgeAt => {
                            if purge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("purgeAt"));
                            }
                            purge__ = map_.next_value::<::std::option::Option<_>>()?.map(delete_bucket_request::Purge::PurgeAt)
;
                        }
                        GeneratedField::Ttl => {
                            if purge__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            purge__ = map_.next_value::<::std::option::Option<_>>()?.map(delete_bucket_request::Purge::Ttl)
;
                        }
                    }
                }
                Ok(DeleteBucketRequest {
                    id: id__.unwrap_or_default(),
                    purge: purge__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.DeleteBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBucketByNameRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if !self.name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.GetBucketByNameRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBucketByNameRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            Name,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBucketByNameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.GetBucketByNameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBucketByNameRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBucketByNameRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.GetBucketByNameRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.GetBucketRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.GetBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBucketRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.GetBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleAbortIncompleteMultipartUpload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.days_after_initiation != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleAbortIncompleteMultipartUpload", len)?;
        if self.days_after_initiation != 0 {
            struct_ser.serialize_field("daysAfterInitiation", &self.days_after_initiation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleAbortIncompleteMultipartUpload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "days_after_initiation",
            "daysAfterInitiation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DaysAfterInitiation,
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
                            "daysAfterInitiation" | "days_after_initiation" => Ok(GeneratedField::DaysAfterInitiation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleAbortIncompleteMultipartUpload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleAbortIncompleteMultipartUpload")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleAbortIncompleteMultipartUpload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut days_after_initiation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::DaysAfterInitiation => {
                            if days_after_initiation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("daysAfterInitiation"));
                            }
                            days_after_initiation__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LifecycleAbortIncompleteMultipartUpload {
                    days_after_initiation: days_after_initiation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleAbortIncompleteMultipartUpload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleConfiguration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rules.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleConfiguration", len)?;
        if !self.rules.is_empty() {
            struct_ser.serialize_field("rules", &self.rules)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleConfiguration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rules",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rules,
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
                            "rules" => Ok(GeneratedField::Rules),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleConfiguration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleConfiguration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleConfiguration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut rules__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Rules => {
                            if rules__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rules"));
                            }
                            rules__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(LifecycleConfiguration {
                    rules: rules__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleConfiguration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleExpiration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.expired_object_delete_marker {
            len += 1;
        }
        if self.expired_with.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleExpiration", len)?;
        if self.expired_object_delete_marker {
            struct_ser.serialize_field("expiredObjectDeleteMarker", &self.expired_object_delete_marker)?;
        }
        if let Some(v) = self.expired_with.as_ref() {
            match v {
                lifecycle_expiration::ExpiredWith::Date(v) => {
                    struct_ser.serialize_field("date", v)?;
                }
                lifecycle_expiration::ExpiredWith::Days(v) => {
                    struct_ser.serialize_field("days", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleExpiration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "expired_object_delete_marker",
            "expiredObjectDeleteMarker",
            "date",
            "days",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExpiredObjectDeleteMarker,
            Date,
            Days,
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
                            "expiredObjectDeleteMarker" | "expired_object_delete_marker" => Ok(GeneratedField::ExpiredObjectDeleteMarker),
                            "date" => Ok(GeneratedField::Date),
                            "days" => Ok(GeneratedField::Days),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleExpiration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleExpiration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleExpiration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut expired_object_delete_marker__ = None;
                let mut expired_with__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ExpiredObjectDeleteMarker => {
                            if expired_object_delete_marker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiredObjectDeleteMarker"));
                            }
                            expired_object_delete_marker__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Date => {
                            if expired_with__.is_some() {
                                return Err(serde::de::Error::duplicate_field("date"));
                            }
                            expired_with__ = map_.next_value::<::std::option::Option<_>>()?.map(lifecycle_expiration::ExpiredWith::Date)
;
                        }
                        GeneratedField::Days => {
                            if expired_with__.is_some() {
                                return Err(serde::de::Error::duplicate_field("days"));
                            }
                            expired_with__ = map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| lifecycle_expiration::ExpiredWith::Days(x.0));
                        }
                    }
                }
                Ok(LifecycleExpiration {
                    expired_object_delete_marker: expired_object_delete_marker__.unwrap_or_default(),
                    expired_with: expired_with__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleExpiration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleFilter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.prefix.is_empty() {
            len += 1;
        }
        if self.object_size_greater_than_bytes != 0 {
            len += 1;
        }
        if self.object_size_less_than_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleFilter", len)?;
        if !self.prefix.is_empty() {
            struct_ser.serialize_field("prefix", &self.prefix)?;
        }
        if self.object_size_greater_than_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectSizeGreaterThanBytes", ToString::to_string(&self.object_size_greater_than_bytes).as_str())?;
        }
        if self.object_size_less_than_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("objectSizeLessThanBytes", ToString::to_string(&self.object_size_less_than_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleFilter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "prefix",
            "object_size_greater_than_bytes",
            "objectSizeGreaterThanBytes",
            "object_size_less_than_bytes",
            "objectSizeLessThanBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Prefix,
            ObjectSizeGreaterThanBytes,
            ObjectSizeLessThanBytes,
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
                            "prefix" => Ok(GeneratedField::Prefix),
                            "objectSizeGreaterThanBytes" | "object_size_greater_than_bytes" => Ok(GeneratedField::ObjectSizeGreaterThanBytes),
                            "objectSizeLessThanBytes" | "object_size_less_than_bytes" => Ok(GeneratedField::ObjectSizeLessThanBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleFilter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleFilter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleFilter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut prefix__ = None;
                let mut object_size_greater_than_bytes__ = None;
                let mut object_size_less_than_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Prefix => {
                            if prefix__.is_some() {
                                return Err(serde::de::Error::duplicate_field("prefix"));
                            }
                            prefix__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ObjectSizeGreaterThanBytes => {
                            if object_size_greater_than_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectSizeGreaterThanBytes"));
                            }
                            object_size_greater_than_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ObjectSizeLessThanBytes => {
                            if object_size_less_than_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectSizeLessThanBytes"));
                            }
                            object_size_less_than_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LifecycleFilter {
                    prefix: prefix__.unwrap_or_default(),
                    object_size_greater_than_bytes: object_size_greater_than_bytes__.unwrap_or_default(),
                    object_size_less_than_bytes: object_size_less_than_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleFilter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleNoncurrentVersionExpiration {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.newer_noncurrent_versions.is_some() {
            len += 1;
        }
        if self.noncurrent_days != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleNoncurrentVersionExpiration", len)?;
        if let Some(v) = self.newer_noncurrent_versions.as_ref() {
            struct_ser.serialize_field("newerNoncurrentVersions", v)?;
        }
        if self.noncurrent_days != 0 {
            struct_ser.serialize_field("noncurrentDays", &self.noncurrent_days)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleNoncurrentVersionExpiration {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "newer_noncurrent_versions",
            "newerNoncurrentVersions",
            "noncurrent_days",
            "noncurrentDays",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NewerNoncurrentVersions,
            NoncurrentDays,
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
                            "newerNoncurrentVersions" | "newer_noncurrent_versions" => Ok(GeneratedField::NewerNoncurrentVersions),
                            "noncurrentDays" | "noncurrent_days" => Ok(GeneratedField::NoncurrentDays),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleNoncurrentVersionExpiration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleNoncurrentVersionExpiration")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleNoncurrentVersionExpiration, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut newer_noncurrent_versions__ = None;
                let mut noncurrent_days__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::NewerNoncurrentVersions => {
                            if newer_noncurrent_versions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newerNoncurrentVersions"));
                            }
                            newer_noncurrent_versions__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::NoncurrentDays => {
                            if noncurrent_days__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noncurrentDays"));
                            }
                            noncurrent_days__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(LifecycleNoncurrentVersionExpiration {
                    newer_noncurrent_versions: newer_noncurrent_versions__,
                    noncurrent_days: noncurrent_days__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleNoncurrentVersionExpiration", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for LifecycleRule {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.filter.is_some() {
            len += 1;
        }
        if self.expiration.is_some() {
            len += 1;
        }
        if self.noncurrent_version_expiration.is_some() {
            len += 1;
        }
        if self.abort_incomplete_multipart_upload.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.LifecycleRule", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if self.status != 0 {
            let v = lifecycle_rule::Status::try_from(self.status)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if let Some(v) = self.filter.as_ref() {
            struct_ser.serialize_field("filter", v)?;
        }
        if let Some(v) = self.expiration.as_ref() {
            struct_ser.serialize_field("expiration", v)?;
        }
        if let Some(v) = self.noncurrent_version_expiration.as_ref() {
            struct_ser.serialize_field("noncurrentVersionExpiration", v)?;
        }
        if let Some(v) = self.abort_incomplete_multipart_upload.as_ref() {
            struct_ser.serialize_field("abortIncompleteMultipartUpload", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LifecycleRule {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "status",
            "filter",
            "expiration",
            "noncurrent_version_expiration",
            "noncurrentVersionExpiration",
            "abort_incomplete_multipart_upload",
            "abortIncompleteMultipartUpload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Status,
            Filter,
            Expiration,
            NoncurrentVersionExpiration,
            AbortIncompleteMultipartUpload,
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
                            "id" => Ok(GeneratedField::Id),
                            "status" => Ok(GeneratedField::Status),
                            "filter" => Ok(GeneratedField::Filter),
                            "expiration" => Ok(GeneratedField::Expiration),
                            "noncurrentVersionExpiration" | "noncurrent_version_expiration" => Ok(GeneratedField::NoncurrentVersionExpiration),
                            "abortIncompleteMultipartUpload" | "abort_incomplete_multipart_upload" => Ok(GeneratedField::AbortIncompleteMultipartUpload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LifecycleRule;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.LifecycleRule")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<LifecycleRule, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut status__ = None;
                let mut filter__ = None;
                let mut expiration__ = None;
                let mut noncurrent_version_expiration__ = None;
                let mut abort_incomplete_multipart_upload__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map_.next_value::<lifecycle_rule::Status>()? as i32);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = map_.next_value()?;
                        }
                        GeneratedField::Expiration => {
                            if expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("expiration"));
                            }
                            expiration__ = map_.next_value()?;
                        }
                        GeneratedField::NoncurrentVersionExpiration => {
                            if noncurrent_version_expiration__.is_some() {
                                return Err(serde::de::Error::duplicate_field("noncurrentVersionExpiration"));
                            }
                            noncurrent_version_expiration__ = map_.next_value()?;
                        }
                        GeneratedField::AbortIncompleteMultipartUpload => {
                            if abort_incomplete_multipart_upload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abortIncompleteMultipartUpload"));
                            }
                            abort_incomplete_multipart_upload__ = map_.next_value()?;
                        }
                    }
                }
                Ok(LifecycleRule {
                    id: id__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    filter: filter__,
                    expiration: expiration__,
                    noncurrent_version_expiration: noncurrent_version_expiration__,
                    abort_incomplete_multipart_upload: abort_incomplete_multipart_upload__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.LifecycleRule", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for lifecycle_rule::Status {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STATUS_UNSPECIFIED",
            Self::Enabled => "ENABLED",
            Self::Disabled => "DISABLED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for lifecycle_rule::Status {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATUS_UNSPECIFIED",
            "ENABLED",
            "DISABLED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = lifecycle_rule::Status;

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
                    "STATUS_UNSPECIFIED" => Ok(lifecycle_rule::Status::Unspecified),
                    "ENABLED" => Ok(lifecycle_rule::Status::Enabled),
                    "DISABLED" => Ok(lifecycle_rule::Status::Disabled),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ListBucketsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.page_size != 0 {
            len += 1;
        }
        if !self.page_token.is_empty() {
            len += 1;
        }
        if !self.filter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.ListBucketsRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if self.page_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("pageSize", ToString::to_string(&self.page_size).as_str())?;
        }
        if !self.page_token.is_empty() {
            struct_ser.serialize_field("pageToken", &self.page_token)?;
        }
        if !self.filter.is_empty() {
            struct_ser.serialize_field("filter", &self.filter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBucketsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
            "page_size",
            "pageSize",
            "page_token",
            "pageToken",
            "filter",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            PageSize,
            PageToken,
            Filter,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "pageSize" | "page_size" => Ok(GeneratedField::PageSize),
                            "pageToken" | "page_token" => Ok(GeneratedField::PageToken),
                            "filter" => Ok(GeneratedField::Filter),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBucketsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.ListBucketsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBucketsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
                let mut filter__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PageSize => {
                            if page_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageSize"));
                            }
                            page_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PageToken => {
                            if page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pageToken"));
                            }
                            page_token__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Filter => {
                            if filter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filter"));
                            }
                            filter__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBucketsRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                    filter: filter__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.ListBucketsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBucketsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.items.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.ListBucketsResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBucketsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "items",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Items,
            NextPageToken,
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
                            "items" => Ok(GeneratedField::Items),
                            "nextPageToken" | "next_page_token" => Ok(GeneratedField::NextPageToken),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBucketsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.ListBucketsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBucketsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut items__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Items => {
                            if items__.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            items__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBucketsResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.ListBucketsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NonCurrentBucketCounters {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.simple_objects_quantity != 0 {
            len += 1;
        }
        if self.simple_objects_size != 0 {
            len += 1;
        }
        if self.multipart_objects_quantity != 0 {
            len += 1;
        }
        if self.multipart_objects_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.NonCurrentBucketCounters", len)?;
        if self.simple_objects_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("simpleObjectsQuantity", ToString::to_string(&self.simple_objects_quantity).as_str())?;
        }
        if self.simple_objects_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("simpleObjectsSize", ToString::to_string(&self.simple_objects_size).as_str())?;
        }
        if self.multipart_objects_quantity != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("multipartObjectsQuantity", ToString::to_string(&self.multipart_objects_quantity).as_str())?;
        }
        if self.multipart_objects_size != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("multipartObjectsSize", ToString::to_string(&self.multipart_objects_size).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NonCurrentBucketCounters {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "simple_objects_quantity",
            "simpleObjectsQuantity",
            "simple_objects_size",
            "simpleObjectsSize",
            "multipart_objects_quantity",
            "multipartObjectsQuantity",
            "multipart_objects_size",
            "multipartObjectsSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            SimpleObjectsQuantity,
            SimpleObjectsSize,
            MultipartObjectsQuantity,
            MultipartObjectsSize,
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
                            "simpleObjectsQuantity" | "simple_objects_quantity" => Ok(GeneratedField::SimpleObjectsQuantity),
                            "simpleObjectsSize" | "simple_objects_size" => Ok(GeneratedField::SimpleObjectsSize),
                            "multipartObjectsQuantity" | "multipart_objects_quantity" => Ok(GeneratedField::MultipartObjectsQuantity),
                            "multipartObjectsSize" | "multipart_objects_size" => Ok(GeneratedField::MultipartObjectsSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NonCurrentBucketCounters;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.NonCurrentBucketCounters")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NonCurrentBucketCounters, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut simple_objects_quantity__ = None;
                let mut simple_objects_size__ = None;
                let mut multipart_objects_quantity__ = None;
                let mut multipart_objects_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::SimpleObjectsQuantity => {
                            if simple_objects_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simpleObjectsQuantity"));
                            }
                            simple_objects_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SimpleObjectsSize => {
                            if simple_objects_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("simpleObjectsSize"));
                            }
                            simple_objects_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MultipartObjectsQuantity => {
                            if multipart_objects_quantity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multipartObjectsQuantity"));
                            }
                            multipart_objects_quantity__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MultipartObjectsSize => {
                            if multipart_objects_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("multipartObjectsSize"));
                            }
                            multipart_objects_size__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NonCurrentBucketCounters {
                    simple_objects_quantity: simple_objects_quantity__.unwrap_or_default(),
                    simple_objects_size: simple_objects_size__.unwrap_or_default(),
                    multipart_objects_quantity: multipart_objects_quantity__.unwrap_or_default(),
                    multipart_objects_size: multipart_objects_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.NonCurrentBucketCounters", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PurgeBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.PurgeBucketRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PurgeBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PurgeBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.PurgeBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PurgeBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PurgeBucketRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.PurgeBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StorageClass {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "STORAGE_CLASS_UNSPECIFIED",
            Self::Standard => "STANDARD",
            Self::EnhancedThroughput => "ENHANCED_THROUGHPUT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for StorageClass {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STORAGE_CLASS_UNSPECIFIED",
            "STANDARD",
            "ENHANCED_THROUGHPUT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = StorageClass;

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
                    "STORAGE_CLASS_UNSPECIFIED" => Ok(StorageClass::Unspecified),
                    "STANDARD" => Ok(StorageClass::Standard),
                    "ENHANCED_THROUGHPUT" => Ok(StorageClass::EnhancedThroughput),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UndeleteBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.UndeleteBucketRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndeleteBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
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
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UndeleteBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.UndeleteBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UndeleteBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UndeleteBucketRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.UndeleteBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateBucketRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.metadata.is_some() {
            len += 1;
        }
        if self.spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.storage.v1.UpdateBucketRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateBucketRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
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
                            "metadata" => Ok(GeneratedField::Metadata),
                            "spec" => Ok(GeneratedField::Spec),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateBucketRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.storage.v1.UpdateBucketRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateBucketRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Metadata => {
                            if metadata__.is_some() {
                                return Err(serde::de::Error::duplicate_field("metadata"));
                            }
                            metadata__ = map_.next_value()?;
                        }
                        GeneratedField::Spec => {
                            if spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spec"));
                            }
                            spec__ = map_.next_value()?;
                        }
                    }
                }
                Ok(UpdateBucketRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.storage.v1.UpdateBucketRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for VersioningPolicy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VERSIONING_POLICY_UNSPECIFIED",
            Self::Disabled => "DISABLED",
            Self::Enabled => "ENABLED",
            Self::Suspended => "SUSPENDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for VersioningPolicy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VERSIONING_POLICY_UNSPECIFIED",
            "DISABLED",
            "ENABLED",
            "SUSPENDED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VersioningPolicy;

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
                    "VERSIONING_POLICY_UNSPECIFIED" => Ok(VersioningPolicy::Unspecified),
                    "DISABLED" => Ok(VersioningPolicy::Disabled),
                    "ENABLED" => Ok(VersioningPolicy::Enabled),
                    "SUSPENDED" => Ok(VersioningPolicy::Suspended),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}

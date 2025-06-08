// @generated
impl serde::Serialize for Backup {
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
        if !self.source_cluster_id.is_empty() {
            len += 1;
        }
        if self.creation_start.is_some() {
            len += 1;
        }
        if self.creation_finish.is_some() {
            len += 1;
        }
        if !self.source_cluster_name.is_empty() {
            len += 1;
        }
        if self.source_cluster_visible {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.Backup", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.source_cluster_id.is_empty() {
            struct_ser.serialize_field("sourceClusterId", &self.source_cluster_id)?;
        }
        if let Some(v) = self.creation_start.as_ref() {
            struct_ser.serialize_field("creationStart", v)?;
        }
        if let Some(v) = self.creation_finish.as_ref() {
            struct_ser.serialize_field("creationFinish", v)?;
        }
        if !self.source_cluster_name.is_empty() {
            struct_ser.serialize_field("sourceClusterName", &self.source_cluster_name)?;
        }
        if self.source_cluster_visible {
            struct_ser.serialize_field("sourceClusterVisible", &self.source_cluster_visible)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Backup {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "source_cluster_id",
            "sourceClusterId",
            "creation_start",
            "creationStart",
            "creation_finish",
            "creationFinish",
            "source_cluster_name",
            "sourceClusterName",
            "source_cluster_visible",
            "sourceClusterVisible",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            SourceClusterId,
            CreationStart,
            CreationFinish,
            SourceClusterName,
            SourceClusterVisible,
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
                            "sourceClusterId" | "source_cluster_id" => Ok(GeneratedField::SourceClusterId),
                            "creationStart" | "creation_start" => Ok(GeneratedField::CreationStart),
                            "creationFinish" | "creation_finish" => Ok(GeneratedField::CreationFinish),
                            "sourceClusterName" | "source_cluster_name" => Ok(GeneratedField::SourceClusterName),
                            "sourceClusterVisible" | "source_cluster_visible" => Ok(GeneratedField::SourceClusterVisible),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Backup;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.Backup")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Backup, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut source_cluster_id__ = None;
                let mut creation_start__ = None;
                let mut creation_finish__ = None;
                let mut source_cluster_name__ = None;
                let mut source_cluster_visible__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceClusterId => {
                            if source_cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceClusterId"));
                            }
                            source_cluster_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::CreationStart => {
                            if creation_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationStart"));
                            }
                            creation_start__ = map_.next_value()?;
                        }
                        GeneratedField::CreationFinish => {
                            if creation_finish__.is_some() {
                                return Err(serde::de::Error::duplicate_field("creationFinish"));
                            }
                            creation_finish__ = map_.next_value()?;
                        }
                        GeneratedField::SourceClusterName => {
                            if source_cluster_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceClusterName"));
                            }
                            source_cluster_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceClusterVisible => {
                            if source_cluster_visible__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceClusterVisible"));
                            }
                            source_cluster_visible__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Backup {
                    id: id__.unwrap_or_default(),
                    source_cluster_id: source_cluster_id__.unwrap_or_default(),
                    creation_start: creation_start__,
                    creation_finish: creation_finish__,
                    source_cluster_name: source_cluster_name__.unwrap_or_default(),
                    source_cluster_visible: source_cluster_visible__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.Backup", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BackupSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backup_window_start.is_empty() {
            len += 1;
        }
        if !self.retention_policy.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.BackupSpec", len)?;
        if !self.backup_window_start.is_empty() {
            struct_ser.serialize_field("backupWindowStart", &self.backup_window_start)?;
        }
        if !self.retention_policy.is_empty() {
            struct_ser.serialize_field("retentionPolicy", &self.retention_policy)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BackupSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "backup_window_start",
            "backupWindowStart",
            "retention_policy",
            "retentionPolicy",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BackupWindowStart,
            RetentionPolicy,
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
                            "backupWindowStart" | "backup_window_start" => Ok(GeneratedField::BackupWindowStart),
                            "retentionPolicy" | "retention_policy" => Ok(GeneratedField::RetentionPolicy),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BackupSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.BackupSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BackupSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut backup_window_start__ = None;
                let mut retention_policy__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::BackupWindowStart => {
                            if backup_window_start__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backupWindowStart"));
                            }
                            backup_window_start__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetentionPolicy => {
                            if retention_policy__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retentionPolicy"));
                            }
                            retention_policy__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BackupSpec {
                    backup_window_start: backup_window_start__.unwrap_or_default(),
                    retention_policy: retention_policy__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.BackupSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BootstrapSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_name.is_empty() {
            len += 1;
        }
        if !self.user_password.is_empty() {
            len += 1;
        }
        if !self.db_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.BootstrapSpec", len)?;
        if !self.user_name.is_empty() {
            struct_ser.serialize_field("userName", &self.user_name)?;
        }
        if !self.user_password.is_empty() {
            struct_ser.serialize_field("userPassword", &self.user_password)?;
        }
        if !self.db_name.is_empty() {
            struct_ser.serialize_field("dbName", &self.db_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BootstrapSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "user_name",
            "userName",
            "user_password",
            "userPassword",
            "db_name",
            "dbName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserName,
            UserPassword,
            DbName,
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
                            "userName" | "user_name" => Ok(GeneratedField::UserName),
                            "userPassword" | "user_password" => Ok(GeneratedField::UserPassword),
                            "dbName" | "db_name" => Ok(GeneratedField::DbName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BootstrapSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.BootstrapSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BootstrapSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut user_name__ = None;
                let mut user_password__ = None;
                let mut db_name__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::UserName => {
                            if user_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userName"));
                            }
                            user_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UserPassword => {
                            if user_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("userPassword"));
                            }
                            user_password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::DbName => {
                            if db_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dbName"));
                            }
                            db_name__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BootstrapSpec {
                    user_name: user_name__.unwrap_or_default(),
                    user_password: user_password__.unwrap_or_default(),
                    db_name: db_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.BootstrapSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Cluster {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.Cluster", len)?;
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
impl<'de> serde::Deserialize<'de> for Cluster {
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
            type Value = Cluster;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.Cluster")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Cluster, V::Error>
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
                Ok(Cluster {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.Cluster", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.network_id.is_empty() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        if self.bootstrap.is_some() {
            len += 1;
        }
        if self.backup.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ClusterSpec", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.network_id.is_empty() {
            struct_ser.serialize_field("networkId", &self.network_id)?;
        }
        if let Some(v) = self.config.as_ref() {
            struct_ser.serialize_field("config", v)?;
        }
        if let Some(v) = self.bootstrap.as_ref() {
            struct_ser.serialize_field("bootstrap", v)?;
        }
        if let Some(v) = self.backup.as_ref() {
            struct_ser.serialize_field("backup", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "network_id",
            "networkId",
            "config",
            "bootstrap",
            "backup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            NetworkId,
            Config,
            Bootstrap,
            Backup,
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
                            "description" => Ok(GeneratedField::Description),
                            "networkId" | "network_id" => Ok(GeneratedField::NetworkId),
                            "config" => Ok(GeneratedField::Config),
                            "bootstrap" => Ok(GeneratedField::Bootstrap),
                            "backup" => Ok(GeneratedField::Backup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ClusterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut network_id__ = None;
                let mut config__ = None;
                let mut bootstrap__ = None;
                let mut backup__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkId => {
                            if network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkId"));
                            }
                            network_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Config => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("config"));
                            }
                            config__ = map_.next_value()?;
                        }
                        GeneratedField::Bootstrap => {
                            if bootstrap__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bootstrap"));
                            }
                            bootstrap__ = map_.next_value()?;
                        }
                        GeneratedField::Backup => {
                            if backup__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backup"));
                            }
                            backup__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterSpec {
                    description: description__.unwrap_or_default(),
                    network_id: network_id__.unwrap_or_default(),
                    config: config__,
                    bootstrap: bootstrap__,
                    backup: backup__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ClusterSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ClusterStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.phase != 0 {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if self.preset_details.is_some() {
            len += 1;
        }
        if self.connection_endpoints.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ClusterStatus", len)?;
        if self.phase != 0 {
            let v = super::super::v1alpha1::cluster_status::Phase::try_from(self.phase)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.phase)))?;
            struct_ser.serialize_field("phase", &v)?;
        }
        if self.state != 0 {
            let v = super::super::v1alpha1::cluster_status::State::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if let Some(v) = self.preset_details.as_ref() {
            struct_ser.serialize_field("presetDetails", v)?;
        }
        if let Some(v) = self.connection_endpoints.as_ref() {
            struct_ser.serialize_field("connectionEndpoints", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ClusterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phase",
            "state",
            "preset_details",
            "presetDetails",
            "connection_endpoints",
            "connectionEndpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            State,
            PresetDetails,
            ConnectionEndpoints,
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
                            "phase" => Ok(GeneratedField::Phase),
                            "state" => Ok(GeneratedField::State),
                            "presetDetails" | "preset_details" => Ok(GeneratedField::PresetDetails),
                            "connectionEndpoints" | "connection_endpoints" => Ok(GeneratedField::ConnectionEndpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ClusterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ClusterStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut state__ = None;
                let mut preset_details__ = None;
                let mut connection_endpoints__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Phase => {
                            if phase__.is_some() {
                                return Err(serde::de::Error::duplicate_field("phase"));
                            }
                            phase__ = Some(map_.next_value::<super::super::v1alpha1::cluster_status::Phase>()? as i32);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<super::super::v1alpha1::cluster_status::State>()? as i32);
                        }
                        GeneratedField::PresetDetails => {
                            if preset_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("presetDetails"));
                            }
                            preset_details__ = map_.next_value()?;
                        }
                        GeneratedField::ConnectionEndpoints => {
                            if connection_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionEndpoints"));
                            }
                            connection_endpoints__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterStatus {
                    phase: phase__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    preset_details: preset_details__,
                    connection_endpoints: connection_endpoints__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ClusterStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConfigSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.version.is_empty() {
            len += 1;
        }
        if self.pooler_config.is_some() {
            len += 1;
        }
        if self.public_access {
            len += 1;
        }
        if self.template.is_some() {
            len += 1;
        }
        if self.config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ConfigSpec", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if let Some(v) = self.pooler_config.as_ref() {
            struct_ser.serialize_field("poolerConfig", v)?;
        }
        if self.public_access {
            struct_ser.serialize_field("publicAccess", &self.public_access)?;
        }
        if let Some(v) = self.template.as_ref() {
            struct_ser.serialize_field("template", v)?;
        }
        if let Some(v) = self.config.as_ref() {
            match v {
                config_spec::Config::PostgresqlConfig16(v) => {
                    struct_ser.serialize_field("postgresqlConfig16", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConfigSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "pooler_config",
            "poolerConfig",
            "public_access",
            "publicAccess",
            "template",
            "postgresql_config_16",
            "postgresqlConfig16",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            PoolerConfig,
            PublicAccess,
            Template,
            PostgresqlConfig16,
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
                            "version" => Ok(GeneratedField::Version),
                            "poolerConfig" | "pooler_config" => Ok(GeneratedField::PoolerConfig),
                            "publicAccess" | "public_access" => Ok(GeneratedField::PublicAccess),
                            "template" => Ok(GeneratedField::Template),
                            "postgresqlConfig16" | "postgresql_config_16" => Ok(GeneratedField::PostgresqlConfig16),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConfigSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ConfigSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConfigSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut pooler_config__ = None;
                let mut public_access__ = None;
                let mut template__ = None;
                let mut config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PoolerConfig => {
                            if pooler_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolerConfig"));
                            }
                            pooler_config__ = map_.next_value()?;
                        }
                        GeneratedField::PublicAccess => {
                            if public_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicAccess"));
                            }
                            public_access__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Template => {
                            if template__.is_some() {
                                return Err(serde::de::Error::duplicate_field("template"));
                            }
                            template__ = map_.next_value()?;
                        }
                        GeneratedField::PostgresqlConfig16 => {
                            if config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("postgresqlConfig16"));
                            }
                            config__ = map_.next_value::<::std::option::Option<_>>()?.map(config_spec::Config::PostgresqlConfig16)
;
                        }
                    }
                }
                Ok(ConfigSpec {
                    version: version__.unwrap_or_default(),
                    pooler_config: pooler_config__,
                    public_access: public_access__.unwrap_or_default(),
                    template: template__,
                    config: config__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ConfigSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ConnectionPoolerConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pooling_mode != 0 {
            len += 1;
        }
        if self.max_pool_size.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ConnectionPoolerConfig", len)?;
        if self.pooling_mode != 0 {
            let v = connection_pooler_config::PoolingMode::try_from(self.pooling_mode)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.pooling_mode)))?;
            struct_ser.serialize_field("poolingMode", &v)?;
        }
        if let Some(v) = self.max_pool_size.as_ref() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("maxPoolSize", ToString::to_string(&v).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ConnectionPoolerConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pooling_mode",
            "poolingMode",
            "max_pool_size",
            "maxPoolSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PoolingMode,
            MaxPoolSize,
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
                            "poolingMode" | "pooling_mode" => Ok(GeneratedField::PoolingMode),
                            "maxPoolSize" | "max_pool_size" => Ok(GeneratedField::MaxPoolSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ConnectionPoolerConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ConnectionPoolerConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ConnectionPoolerConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pooling_mode__ = None;
                let mut max_pool_size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PoolingMode => {
                            if pooling_mode__.is_some() {
                                return Err(serde::de::Error::duplicate_field("poolingMode"));
                            }
                            pooling_mode__ = Some(map_.next_value::<connection_pooler_config::PoolingMode>()? as i32);
                        }
                        GeneratedField::MaxPoolSize => {
                            if max_pool_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxPoolSize"));
                            }
                            max_pool_size__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                    }
                }
                Ok(ConnectionPoolerConfig {
                    pooling_mode: pooling_mode__.unwrap_or_default(),
                    max_pool_size: max_pool_size__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ConnectionPoolerConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for connection_pooler_config::PoolingMode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "POOLING_MODE_UNSPECIFIED",
            Self::Session => "SESSION",
            Self::Transaction => "TRANSACTION",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for connection_pooler_config::PoolingMode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "POOLING_MODE_UNSPECIFIED",
            "SESSION",
            "TRANSACTION",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = connection_pooler_config::PoolingMode;

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
                    "POOLING_MODE_UNSPECIFIED" => Ok(connection_pooler_config::PoolingMode::Unspecified),
                    "SESSION" => Ok(connection_pooler_config::PoolingMode::Session),
                    "TRANSACTION" => Ok(connection_pooler_config::PoolingMode::Transaction),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for CreateClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.CreateClusterRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateClusterRequest {
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
            type Value = CreateClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.CreateClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateClusterRequest, V::Error>
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
                Ok(CreateClusterRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.CreateClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.DeleteClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteClusterRequest {
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
            type Value = DeleteClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.DeleteClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteClusterRequest, V::Error>
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
                Ok(DeleteClusterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.DeleteClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Endpoints {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.private_read_write.is_empty() {
            len += 1;
        }
        if !self.private_read_only.is_empty() {
            len += 1;
        }
        if !self.public_read_write.is_empty() {
            len += 1;
        }
        if !self.public_read_only.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.Endpoints", len)?;
        if !self.private_read_write.is_empty() {
            struct_ser.serialize_field("privateReadWrite", &self.private_read_write)?;
        }
        if !self.private_read_only.is_empty() {
            struct_ser.serialize_field("privateReadOnly", &self.private_read_only)?;
        }
        if !self.public_read_write.is_empty() {
            struct_ser.serialize_field("publicReadWrite", &self.public_read_write)?;
        }
        if !self.public_read_only.is_empty() {
            struct_ser.serialize_field("publicReadOnly", &self.public_read_only)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Endpoints {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "private_read_write",
            "privateReadWrite",
            "private_read_only",
            "privateReadOnly",
            "public_read_write",
            "publicReadWrite",
            "public_read_only",
            "publicReadOnly",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PrivateReadWrite,
            PrivateReadOnly,
            PublicReadWrite,
            PublicReadOnly,
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
                            "privateReadWrite" | "private_read_write" => Ok(GeneratedField::PrivateReadWrite),
                            "privateReadOnly" | "private_read_only" => Ok(GeneratedField::PrivateReadOnly),
                            "publicReadWrite" | "public_read_write" => Ok(GeneratedField::PublicReadWrite),
                            "publicReadOnly" | "public_read_only" => Ok(GeneratedField::PublicReadOnly),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Endpoints;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.Endpoints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Endpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut private_read_write__ = None;
                let mut private_read_only__ = None;
                let mut public_read_write__ = None;
                let mut public_read_only__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::PrivateReadWrite => {
                            if private_read_write__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateReadWrite"));
                            }
                            private_read_write__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PrivateReadOnly => {
                            if private_read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("privateReadOnly"));
                            }
                            private_read_only__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicReadWrite => {
                            if public_read_write__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicReadWrite"));
                            }
                            public_read_write__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicReadOnly => {
                            if public_read_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicReadOnly"));
                            }
                            public_read_only__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Endpoints {
                    private_read_write: private_read_write__.unwrap_or_default(),
                    private_read_only: private_read_only__.unwrap_or_default(),
                    public_read_write: public_read_write__.unwrap_or_default(),
                    public_read_only: public_read_only__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.Endpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBackupRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_id.is_empty() {
            len += 1;
        }
        if !self.backup_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.GetBackupRequest", len)?;
        if !self.cluster_id.is_empty() {
            struct_ser.serialize_field("clusterId", &self.cluster_id)?;
        }
        if !self.backup_id.is_empty() {
            struct_ser.serialize_field("backupId", &self.backup_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBackupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_id",
            "clusterId",
            "backup_id",
            "backupId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterId,
            BackupId,
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
                            "clusterId" | "cluster_id" => Ok(GeneratedField::ClusterId),
                            "backupId" | "backup_id" => Ok(GeneratedField::BackupId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBackupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.GetBackupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetBackupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_id__ = None;
                let mut backup_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterId => {
                            if cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterId"));
                            }
                            cluster_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackupId => {
                            if backup_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backupId"));
                            }
                            backup_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetBackupRequest {
                    cluster_id: cluster_id__.unwrap_or_default(),
                    backup_id: backup_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.GetBackupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClusterForBackupRequest {
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
        if !self.backup_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.GetClusterForBackupRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.backup_id.is_empty() {
            struct_ser.serialize_field("backupId", &self.backup_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClusterForBackupRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "backup_id",
            "backupId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            BackupId,
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
                            "backupId" | "backup_id" => Ok(GeneratedField::BackupId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetClusterForBackupRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.GetClusterForBackupRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClusterForBackupRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut backup_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::BackupId => {
                            if backup_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backupId"));
                            }
                            backup_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetClusterForBackupRequest {
                    id: id__.unwrap_or_default(),
                    backup_id: backup_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.GetClusterForBackupRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.GetClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClusterRequest {
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
            type Value = GetClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.GetClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClusterRequest, V::Error>
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
                Ok(GetClusterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.GetClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBackupsByClusterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cluster_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsByClusterRequest", len)?;
        if !self.cluster_id.is_empty() {
            struct_ser.serialize_field("clusterId", &self.cluster_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBackupsByClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cluster_id",
            "clusterId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ClusterId,
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
                            "clusterId" | "cluster_id" => Ok(GeneratedField::ClusterId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBackupsByClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ListBackupsByClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBackupsByClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cluster_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ClusterId => {
                            if cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusterId"));
                            }
                            cluster_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBackupsByClusterRequest {
                    cluster_id: cluster_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsByClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBackupsRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBackupsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "parent_id",
            "parentId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBackupsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ListBackupsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBackupsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBackupsRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListBackupsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.backups.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsResponse", len)?;
        if !self.backups.is_empty() {
            struct_ser.serialize_field("backups", &self.backups)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListBackupsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "backups",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Backups,
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
                            "backups" => Ok(GeneratedField::Backups),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListBackupsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ListBackupsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListBackupsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut backups__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Backups => {
                            if backups__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backups"));
                            }
                            backups__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListBackupsResponse {
                    backups: backups__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ListBackupsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClustersRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ListClustersRequest", len)?;
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
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClustersRequest {
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
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ParentId,
            PageSize,
            PageToken,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ListClustersRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ListClustersRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClustersRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut parent_id__ = None;
                let mut page_size__ = None;
                let mut page_token__ = None;
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
                    }
                }
                Ok(ListClustersRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ListClustersRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListClustersResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.clusters.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.ListClustersResponse", len)?;
        if !self.clusters.is_empty() {
            struct_ser.serialize_field("clusters", &self.clusters)?;
        }
        if !self.next_page_token.is_empty() {
            struct_ser.serialize_field("nextPageToken", &self.next_page_token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListClustersResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "clusters",
            "next_page_token",
            "nextPageToken",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Clusters,
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
                            "clusters" => Ok(GeneratedField::Clusters),
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
            type Value = ListClustersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.ListClustersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClustersResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut clusters__ = None;
                let mut next_page_token__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Clusters => {
                            if clusters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("clusters"));
                            }
                            clusters__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NextPageToken => {
                            if next_page_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextPageToken"));
                            }
                            next_page_token__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ListClustersResponse {
                    clusters: clusters__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.ListClustersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestoreClusterRequest {
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
        if !self.backup_id.is_empty() {
            len += 1;
        }
        if !self.source_cluster_id.is_empty() {
            len += 1;
        }
        if self.recovery_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.RestoreClusterRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        if !self.backup_id.is_empty() {
            struct_ser.serialize_field("backupId", &self.backup_id)?;
        }
        if !self.source_cluster_id.is_empty() {
            struct_ser.serialize_field("sourceClusterId", &self.source_cluster_id)?;
        }
        if let Some(v) = self.recovery_time.as_ref() {
            struct_ser.serialize_field("recoveryTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestoreClusterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "metadata",
            "spec",
            "backup_id",
            "backupId",
            "source_cluster_id",
            "sourceClusterId",
            "recovery_time",
            "recoveryTime",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Metadata,
            Spec,
            BackupId,
            SourceClusterId,
            RecoveryTime,
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
                            "backupId" | "backup_id" => Ok(GeneratedField::BackupId),
                            "sourceClusterId" | "source_cluster_id" => Ok(GeneratedField::SourceClusterId),
                            "recoveryTime" | "recovery_time" => Ok(GeneratedField::RecoveryTime),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestoreClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.RestoreClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestoreClusterRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut metadata__ = None;
                let mut spec__ = None;
                let mut backup_id__ = None;
                let mut source_cluster_id__ = None;
                let mut recovery_time__ = None;
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
                        GeneratedField::BackupId => {
                            if backup_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("backupId"));
                            }
                            backup_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SourceClusterId => {
                            if source_cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sourceClusterId"));
                            }
                            source_cluster_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RecoveryTime => {
                            if recovery_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recoveryTime"));
                            }
                            recovery_time__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RestoreClusterRequest {
                    metadata: metadata__,
                    spec: spec__,
                    backup_id: backup_id__.unwrap_or_default(),
                    source_cluster_id: source_cluster_id__.unwrap_or_default(),
                    recovery_time: recovery_time__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.RestoreClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StartClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.StartClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StartClusterRequest {
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
            type Value = StartClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.StartClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StartClusterRequest, V::Error>
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
                Ok(StartClusterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.StartClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for StopClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.StopClusterRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for StopClusterRequest {
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
            type Value = StopClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.StopClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<StopClusterRequest, V::Error>
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
                Ok(StopClusterRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.StopClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TemplateSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.resources.is_some() {
            len += 1;
        }
        if self.hosts.is_some() {
            len += 1;
        }
        if self.disk.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.TemplateSpec", len)?;
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        if let Some(v) = self.hosts.as_ref() {
            struct_ser.serialize_field("hosts", v)?;
        }
        if let Some(v) = self.disk.as_ref() {
            struct_ser.serialize_field("disk", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TemplateSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resources",
            "hosts",
            "disk",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Resources,
            Hosts,
            Disk,
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
                            "resources" => Ok(GeneratedField::Resources),
                            "hosts" => Ok(GeneratedField::Hosts),
                            "disk" => Ok(GeneratedField::Disk),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TemplateSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.TemplateSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TemplateSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resources__ = None;
                let mut hosts__ = None;
                let mut disk__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                        GeneratedField::Hosts => {
                            if hosts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            hosts__ = map_.next_value()?;
                        }
                        GeneratedField::Disk => {
                            if disk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disk"));
                            }
                            disk__ = map_.next_value()?;
                        }
                    }
                }
                Ok(TemplateSpec {
                    resources: resources__,
                    hosts: hosts__,
                    disk: disk__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.TemplateSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateClusterRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.postgresql.v1alpha1.UpdateClusterRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateClusterRequest {
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
            type Value = UpdateClusterRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.postgresql.v1alpha1.UpdateClusterRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateClusterRequest, V::Error>
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
                Ok(UpdateClusterRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.postgresql.v1alpha1.UpdateClusterRequest", FIELDS, GeneratedVisitor)
    }
}

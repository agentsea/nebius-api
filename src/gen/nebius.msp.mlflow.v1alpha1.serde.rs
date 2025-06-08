// @generated
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.Cluster", len)?;
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.Cluster")
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
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.Cluster", FIELDS, GeneratedVisitor)
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
        if self.public_access {
            len += 1;
        }
        if !self.admin_username.is_empty() {
            len += 1;
        }
        if !self.admin_password.is_empty() {
            len += 1;
        }
        if !self.service_account_id.is_empty() {
            len += 1;
        }
        if !self.storage_bucket_name.is_empty() {
            len += 1;
        }
        if !self.network_id.is_empty() {
            len += 1;
        }
        if !self.size.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.ClusterSpec", len)?;
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if self.public_access {
            struct_ser.serialize_field("publicAccess", &self.public_access)?;
        }
        if !self.admin_username.is_empty() {
            struct_ser.serialize_field("adminUsername", &self.admin_username)?;
        }
        if !self.admin_password.is_empty() {
            struct_ser.serialize_field("adminPassword", &self.admin_password)?;
        }
        if !self.service_account_id.is_empty() {
            struct_ser.serialize_field("serviceAccountId", &self.service_account_id)?;
        }
        if !self.storage_bucket_name.is_empty() {
            struct_ser.serialize_field("storageBucketName", &self.storage_bucket_name)?;
        }
        if !self.network_id.is_empty() {
            struct_ser.serialize_field("networkId", &self.network_id)?;
        }
        if !self.size.is_empty() {
            struct_ser.serialize_field("size", &self.size)?;
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
            "public_access",
            "publicAccess",
            "admin_username",
            "adminUsername",
            "admin_password",
            "adminPassword",
            "service_account_id",
            "serviceAccountId",
            "storage_bucket_name",
            "storageBucketName",
            "network_id",
            "networkId",
            "size",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            PublicAccess,
            AdminUsername,
            AdminPassword,
            ServiceAccountId,
            StorageBucketName,
            NetworkId,
            Size,
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
                            "publicAccess" | "public_access" => Ok(GeneratedField::PublicAccess),
                            "adminUsername" | "admin_username" => Ok(GeneratedField::AdminUsername),
                            "adminPassword" | "admin_password" => Ok(GeneratedField::AdminPassword),
                            "serviceAccountId" | "service_account_id" => Ok(GeneratedField::ServiceAccountId),
                            "storageBucketName" | "storage_bucket_name" => Ok(GeneratedField::StorageBucketName),
                            "networkId" | "network_id" => Ok(GeneratedField::NetworkId),
                            "size" => Ok(GeneratedField::Size),
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.ClusterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut public_access__ = None;
                let mut admin_username__ = None;
                let mut admin_password__ = None;
                let mut service_account_id__ = None;
                let mut storage_bucket_name__ = None;
                let mut network_id__ = None;
                let mut size__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map_.next_value()?);
                        }
                        GeneratedField::PublicAccess => {
                            if public_access__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicAccess"));
                            }
                            public_access__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdminUsername => {
                            if admin_username__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminUsername"));
                            }
                            admin_username__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AdminPassword => {
                            if admin_password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("adminPassword"));
                            }
                            admin_password__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ServiceAccountId => {
                            if service_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAccountId"));
                            }
                            service_account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::StorageBucketName => {
                            if storage_bucket_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("storageBucketName"));
                            }
                            storage_bucket_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkId => {
                            if network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkId"));
                            }
                            network_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterSpec {
                    description: description__.unwrap_or_default(),
                    public_access: public_access__.unwrap_or_default(),
                    admin_username: admin_username__.unwrap_or_default(),
                    admin_password: admin_password__.unwrap_or_default(),
                    service_account_id: service_account_id__.unwrap_or_default(),
                    storage_bucket_name: storage_bucket_name__.unwrap_or_default(),
                    network_id: network_id__.unwrap_or_default(),
                    size: size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.ClusterSpec", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.CreateClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.CreateClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.CreateClusterRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.DeleteClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.DeleteClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.DeleteClusterRequest", FIELDS, GeneratedVisitor)
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
        if !self.private.is_empty() {
            len += 1;
        }
        if !self.public.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.Endpoints", len)?;
        if !self.private.is_empty() {
            struct_ser.serialize_field("private", &self.private)?;
        }
        if !self.public.is_empty() {
            struct_ser.serialize_field("public", &self.public)?;
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
            "private",
            "public",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Private,
            Public,
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
                            "private" => Ok(GeneratedField::Private),
                            "public" => Ok(GeneratedField::Public),
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.Endpoints")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Endpoints, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut private__ = None;
                let mut public__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Private => {
                            if private__.is_some() {
                                return Err(serde::de::Error::duplicate_field("private"));
                            }
                            private__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Public => {
                            if public__.is_some() {
                                return Err(serde::de::Error::duplicate_field("public"));
                            }
                            public__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Endpoints {
                    private: private__.unwrap_or_default(),
                    public: public__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.Endpoints", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetClusterByNameRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.GetClusterByNameRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetClusterByNameRequest {
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
            type Value = GetClusterByNameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.GetClusterByNameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetClusterByNameRequest, V::Error>
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
                Ok(GetClusterByNameRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.GetClusterByNameRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.GetClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.GetClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.GetClusterRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.ListClustersRequest", len)?;
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
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.ListClustersRequest")
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
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.ListClustersRequest", FIELDS, GeneratedVisitor)
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
        if !self.items.is_empty() {
            len += 1;
        }
        if !self.next_page_token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.ListClustersResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
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
            type Value = ListClustersResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.ListClustersResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListClustersResponse, V::Error>
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
                Ok(ListClustersResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.ListClustersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for MlflowClusterStatus {
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
        if !self.tracking_endpoint.is_empty() {
            len += 1;
        }
        if !self.effective_storage_bucket_name.is_empty() {
            len += 1;
        }
        if self.experiments_count != 0 {
            len += 1;
        }
        if !self.mlflow_version.is_empty() {
            len += 1;
        }
        if self.tracking_endpoints.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.mlflow.v1alpha1.MlflowClusterStatus", len)?;
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
        if !self.tracking_endpoint.is_empty() {
            struct_ser.serialize_field("trackingEndpoint", &self.tracking_endpoint)?;
        }
        if !self.effective_storage_bucket_name.is_empty() {
            struct_ser.serialize_field("effectiveStorageBucketName", &self.effective_storage_bucket_name)?;
        }
        if self.experiments_count != 0 {
            struct_ser.serialize_field("experimentsCount", &self.experiments_count)?;
        }
        if !self.mlflow_version.is_empty() {
            struct_ser.serialize_field("mlflowVersion", &self.mlflow_version)?;
        }
        if let Some(v) = self.tracking_endpoints.as_ref() {
            struct_ser.serialize_field("trackingEndpoints", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MlflowClusterStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phase",
            "state",
            "tracking_endpoint",
            "trackingEndpoint",
            "effective_storage_bucket_name",
            "effectiveStorageBucketName",
            "experiments_count",
            "experimentsCount",
            "mlflow_version",
            "mlflowVersion",
            "tracking_endpoints",
            "trackingEndpoints",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            State,
            TrackingEndpoint,
            EffectiveStorageBucketName,
            ExperimentsCount,
            MlflowVersion,
            TrackingEndpoints,
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
                            "trackingEndpoint" | "tracking_endpoint" => Ok(GeneratedField::TrackingEndpoint),
                            "effectiveStorageBucketName" | "effective_storage_bucket_name" => Ok(GeneratedField::EffectiveStorageBucketName),
                            "experimentsCount" | "experiments_count" => Ok(GeneratedField::ExperimentsCount),
                            "mlflowVersion" | "mlflow_version" => Ok(GeneratedField::MlflowVersion),
                            "trackingEndpoints" | "tracking_endpoints" => Ok(GeneratedField::TrackingEndpoints),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MlflowClusterStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.mlflow.v1alpha1.MlflowClusterStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MlflowClusterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut state__ = None;
                let mut tracking_endpoint__ = None;
                let mut effective_storage_bucket_name__ = None;
                let mut experiments_count__ = None;
                let mut mlflow_version__ = None;
                let mut tracking_endpoints__ = None;
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
                        GeneratedField::TrackingEndpoint => {
                            if tracking_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackingEndpoint"));
                            }
                            tracking_endpoint__ = Some(map_.next_value()?);
                        }
                        GeneratedField::EffectiveStorageBucketName => {
                            if effective_storage_bucket_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("effectiveStorageBucketName"));
                            }
                            effective_storage_bucket_name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ExperimentsCount => {
                            if experiments_count__.is_some() {
                                return Err(serde::de::Error::duplicate_field("experimentsCount"));
                            }
                            experiments_count__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MlflowVersion => {
                            if mlflow_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mlflowVersion"));
                            }
                            mlflow_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TrackingEndpoints => {
                            if tracking_endpoints__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trackingEndpoints"));
                            }
                            tracking_endpoints__ = map_.next_value()?;
                        }
                    }
                }
                Ok(MlflowClusterStatus {
                    phase: phase__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    tracking_endpoint: tracking_endpoint__.unwrap_or_default(),
                    effective_storage_bucket_name: effective_storage_bucket_name__.unwrap_or_default(),
                    experiments_count: experiments_count__.unwrap_or_default(),
                    mlflow_version: mlflow_version__.unwrap_or_default(),
                    tracking_endpoints: tracking_endpoints__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.mlflow.v1alpha1.MlflowClusterStatus", FIELDS, GeneratedVisitor)
    }
}

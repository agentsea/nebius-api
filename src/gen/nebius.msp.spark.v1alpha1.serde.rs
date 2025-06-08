// @generated
impl serde::Serialize for CancelJobRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.CancelJobRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CancelJobRequest {
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
            type Value = CancelJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.CancelJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CancelJobRequest, V::Error>
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
                Ok(CancelJobRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.CancelJobRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.Cluster", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.Cluster")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.Cluster", FIELDS, GeneratedVisitor)
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
        if self.description.is_some() {
            len += 1;
        }
        if self.limits.is_some() {
            len += 1;
        }
        if self.authorization.is_some() {
            len += 1;
        }
        if !self.service_account_id.is_empty() {
            len += 1;
        }
        if !self.network_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ClusterSpec", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.limits.as_ref() {
            struct_ser.serialize_field("limits", v)?;
        }
        if let Some(v) = self.authorization.as_ref() {
            struct_ser.serialize_field("authorization", v)?;
        }
        if !self.service_account_id.is_empty() {
            struct_ser.serialize_field("serviceAccountId", &self.service_account_id)?;
        }
        if !self.network_id.is_empty() {
            struct_ser.serialize_field("networkId", &self.network_id)?;
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
            "limits",
            "authorization",
            "service_account_id",
            "serviceAccountId",
            "network_id",
            "networkId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Limits,
            Authorization,
            ServiceAccountId,
            NetworkId,
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
                            "limits" => Ok(GeneratedField::Limits),
                            "authorization" => Ok(GeneratedField::Authorization),
                            "serviceAccountId" | "service_account_id" => Ok(GeneratedField::ServiceAccountId),
                            "networkId" | "network_id" => Ok(GeneratedField::NetworkId),
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ClusterSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut limits__ = None;
                let mut authorization__ = None;
                let mut service_account_id__ = None;
                let mut network_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Limits => {
                            if limits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limits"));
                            }
                            limits__ = map_.next_value()?;
                        }
                        GeneratedField::Authorization => {
                            if authorization__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorization"));
                            }
                            authorization__ = map_.next_value()?;
                        }
                        GeneratedField::ServiceAccountId => {
                            if service_account_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceAccountId"));
                            }
                            service_account_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::NetworkId => {
                            if network_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("networkId"));
                            }
                            network_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ClusterSpec {
                    description: description__,
                    limits: limits__,
                    authorization: authorization__,
                    service_account_id: service_account_id__.unwrap_or_default(),
                    network_id: network_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ClusterSpec", FIELDS, GeneratedVisitor)
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
        if self.history_server_endpoint.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ClusterStatus", len)?;
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
        if let Some(v) = self.history_server_endpoint.as_ref() {
            struct_ser.serialize_field("historyServerEndpoint", v)?;
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
            "history_server_endpoint",
            "historyServerEndpoint",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            State,
            HistoryServerEndpoint,
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
                            "historyServerEndpoint" | "history_server_endpoint" => Ok(GeneratedField::HistoryServerEndpoint),
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ClusterStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ClusterStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut state__ = None;
                let mut history_server_endpoint__ = None;
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
                        GeneratedField::HistoryServerEndpoint => {
                            if history_server_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("historyServerEndpoint"));
                            }
                            history_server_endpoint__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ClusterStatus {
                    phase: phase__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    history_server_endpoint: history_server_endpoint__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ClusterStatus", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.CreateClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.CreateClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.CreateClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateJobRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.CreateJobRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateJobRequest {
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
            type Value = CreateJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.CreateJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateJobRequest, V::Error>
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
                Ok(CreateJobRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.CreateJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CreateSessionRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.CreateSessionRequest", len)?;
        if let Some(v) = self.metadata.as_ref() {
            struct_ser.serialize_field("metadata", v)?;
        }
        if let Some(v) = self.spec.as_ref() {
            struct_ser.serialize_field("spec", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CreateSessionRequest {
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
            type Value = CreateSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.CreateSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<CreateSessionRequest, V::Error>
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
                Ok(CreateSessionRequest {
                    metadata: metadata__,
                    spec: spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.CreateSessionRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.DeleteClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.DeleteClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.DeleteClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteSessionRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.DeleteSessionRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteSessionRequest {
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
            type Value = DeleteSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.DeleteSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DeleteSessionRequest, V::Error>
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
                Ok(DeleteSessionRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.DeleteSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DriverTemplateSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.disk.is_some() {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.DriverTemplateSpec", len)?;
        if let Some(v) = self.disk.as_ref() {
            struct_ser.serialize_field("disk", v)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DriverTemplateSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disk",
            "resources",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disk,
            Resources,
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
                            "disk" => Ok(GeneratedField::Disk),
                            "resources" => Ok(GeneratedField::Resources),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DriverTemplateSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.DriverTemplateSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DriverTemplateSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disk__ = None;
                let mut resources__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Disk => {
                            if disk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disk"));
                            }
                            disk__ = map_.next_value()?;
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                    }
                }
                Ok(DriverTemplateSpec {
                    disk: disk__,
                    resources: resources__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.DriverTemplateSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DynamicAllocationSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.min != 0 {
            len += 1;
        }
        if self.max != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.DynamicAllocationSpec", len)?;
        if self.min != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("min", ToString::to_string(&self.min).as_str())?;
        }
        if self.max != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("max", ToString::to_string(&self.max).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DynamicAllocationSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "min",
            "max",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Min,
            Max,
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
                            "min" => Ok(GeneratedField::Min),
                            "max" => Ok(GeneratedField::Max),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DynamicAllocationSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.DynamicAllocationSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<DynamicAllocationSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut min__ = None;
                let mut max__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Min => {
                            if min__.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            min__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Max => {
                            if max__.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            max__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(DynamicAllocationSpec {
                    min: min__.unwrap_or_default(),
                    max: max__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.DynamicAllocationSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutorTemplateSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.disk.is_some() {
            len += 1;
        }
        if self.resources.is_some() {
            len += 1;
        }
        if self.hosts_spec.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ExecutorTemplateSpec", len)?;
        if let Some(v) = self.disk.as_ref() {
            struct_ser.serialize_field("disk", v)?;
        }
        if let Some(v) = self.resources.as_ref() {
            struct_ser.serialize_field("resources", v)?;
        }
        if let Some(v) = self.hosts_spec.as_ref() {
            match v {
                executor_template_spec::HostsSpec::Hosts(v) => {
                    struct_ser.serialize_field("hosts", v)?;
                }
                executor_template_spec::HostsSpec::HostsDynamicAllocation(v) => {
                    struct_ser.serialize_field("hostsDynamicAllocation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutorTemplateSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "disk",
            "resources",
            "hosts",
            "hosts_dynamic_allocation",
            "hostsDynamicAllocation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Disk,
            Resources,
            Hosts,
            HostsDynamicAllocation,
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
                            "disk" => Ok(GeneratedField::Disk),
                            "resources" => Ok(GeneratedField::Resources),
                            "hosts" => Ok(GeneratedField::Hosts),
                            "hostsDynamicAllocation" | "hosts_dynamic_allocation" => Ok(GeneratedField::HostsDynamicAllocation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutorTemplateSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ExecutorTemplateSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ExecutorTemplateSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut disk__ = None;
                let mut resources__ = None;
                let mut hosts_spec__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Disk => {
                            if disk__.is_some() {
                                return Err(serde::de::Error::duplicate_field("disk"));
                            }
                            disk__ = map_.next_value()?;
                        }
                        GeneratedField::Resources => {
                            if resources__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resources"));
                            }
                            resources__ = map_.next_value()?;
                        }
                        GeneratedField::Hosts => {
                            if hosts_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hosts"));
                            }
                            hosts_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(executor_template_spec::HostsSpec::Hosts)
;
                        }
                        GeneratedField::HostsDynamicAllocation => {
                            if hosts_spec__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostsDynamicAllocation"));
                            }
                            hosts_spec__ = map_.next_value::<::std::option::Option<_>>()?.map(executor_template_spec::HostsSpec::HostsDynamicAllocation)
;
                        }
                    }
                }
                Ok(ExecutorTemplateSpec {
                    disk: disk__,
                    resources: resources__,
                    hosts_spec: hosts_spec__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ExecutorTemplateSpec", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.GetClusterByNameRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.GetClusterByNameRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.GetClusterByNameRequest", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.GetClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.GetClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.GetClusterRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetJobRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.GetJobRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetJobRequest {
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
            type Value = GetJobRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.GetJobRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetJobRequest, V::Error>
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
                Ok(GetJobRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.GetJobRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSessionByNameRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.GetSessionByNameRequest", len)?;
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSessionByNameRequest {
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
            type Value = GetSessionByNameRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.GetSessionByNameRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSessionByNameRequest, V::Error>
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
                Ok(GetSessionByNameRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    name: name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.GetSessionByNameRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetSessionRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.GetSessionRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetSessionRequest {
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
            type Value = GetSessionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.GetSessionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetSessionRequest, V::Error>
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
                Ok(GetSessionRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.GetSessionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JavaConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.entrypoint_class.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.JavaConfig", len)?;
        if !self.entrypoint_class.is_empty() {
            struct_ser.serialize_field("entrypointClass", &self.entrypoint_class)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JavaConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "entrypoint_class",
            "entrypointClass",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EntrypointClass,
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
                            "entrypointClass" | "entrypoint_class" => Ok(GeneratedField::EntrypointClass),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JavaConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.JavaConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JavaConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut entrypoint_class__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::EntrypointClass => {
                            if entrypoint_class__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entrypointClass"));
                            }
                            entrypoint_class__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(JavaConfig {
                    entrypoint_class: entrypoint_class__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.JavaConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Job {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.Job", len)?;
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
impl<'de> serde::Deserialize<'de> for Job {
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
            type Value = Job;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.Job")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Job, V::Error>
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
                Ok(Job {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.Job", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobResultCode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "JOB_RESULT_CODE_UNSPECIFIED",
            Self::Succeeded => "SUCCEEDED",
            Self::Error => "ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for JobResultCode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JOB_RESULT_CODE_UNSPECIFIED",
            "SUCCEEDED",
            "ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobResultCode;

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
                    "JOB_RESULT_CODE_UNSPECIFIED" => Ok(JobResultCode::Unspecified),
                    "SUCCEEDED" => Ok(JobResultCode::Succeeded),
                    "ERROR" => Ok(JobResultCode::Error),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for JobResultDetails {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.code != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.JobResultDetails", len)?;
        if self.code != 0 {
            let v = JobResultCode::try_from(self.code)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.code)))?;
            struct_ser.serialize_field("code", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobResultDetails {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "code",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Code,
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
                            "code" => Ok(GeneratedField::Code),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobResultDetails;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.JobResultDetails")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobResultDetails, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut code__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value::<JobResultCode>()? as i32);
                        }
                    }
                }
                Ok(JobResultDetails {
                    code: code__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.JobResultDetails", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        if !self.application_file_uri.is_empty() {
            len += 1;
        }
        if self.driver.is_some() {
            len += 1;
        }
        if self.executor.is_some() {
            len += 1;
        }
        if !self.spark_version.is_empty() {
            len += 1;
        }
        if !self.application_args.is_empty() {
            len += 1;
        }
        if !self.file_uris.is_empty() {
            len += 1;
        }
        if !self.jar_uris.is_empty() {
            len += 1;
        }
        if !self.packages.is_empty() {
            len += 1;
        }
        if !self.spark_conf.is_empty() {
            len += 1;
        }
        if self.runtime_config.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.JobSpec", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.application_file_uri.is_empty() {
            struct_ser.serialize_field("applicationFileUri", &self.application_file_uri)?;
        }
        if let Some(v) = self.driver.as_ref() {
            struct_ser.serialize_field("driver", v)?;
        }
        if let Some(v) = self.executor.as_ref() {
            struct_ser.serialize_field("executor", v)?;
        }
        if !self.spark_version.is_empty() {
            struct_ser.serialize_field("sparkVersion", &self.spark_version)?;
        }
        if !self.application_args.is_empty() {
            struct_ser.serialize_field("applicationArgs", &self.application_args)?;
        }
        if !self.file_uris.is_empty() {
            struct_ser.serialize_field("fileUris", &self.file_uris)?;
        }
        if !self.jar_uris.is_empty() {
            struct_ser.serialize_field("jarUris", &self.jar_uris)?;
        }
        if !self.packages.is_empty() {
            struct_ser.serialize_field("packages", &self.packages)?;
        }
        if !self.spark_conf.is_empty() {
            struct_ser.serialize_field("sparkConf", &self.spark_conf)?;
        }
        if let Some(v) = self.runtime_config.as_ref() {
            match v {
                job_spec::RuntimeConfig::Python(v) => {
                    struct_ser.serialize_field("python", v)?;
                }
                job_spec::RuntimeConfig::Java(v) => {
                    struct_ser.serialize_field("java", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "application_file_uri",
            "applicationFileUri",
            "driver",
            "executor",
            "spark_version",
            "sparkVersion",
            "application_args",
            "applicationArgs",
            "file_uris",
            "fileUris",
            "jar_uris",
            "jarUris",
            "packages",
            "spark_conf",
            "sparkConf",
            "python",
            "java",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            ApplicationFileUri,
            Driver,
            Executor,
            SparkVersion,
            ApplicationArgs,
            FileUris,
            JarUris,
            Packages,
            SparkConf,
            Python,
            Java,
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
                            "applicationFileUri" | "application_file_uri" => Ok(GeneratedField::ApplicationFileUri),
                            "driver" => Ok(GeneratedField::Driver),
                            "executor" => Ok(GeneratedField::Executor),
                            "sparkVersion" | "spark_version" => Ok(GeneratedField::SparkVersion),
                            "applicationArgs" | "application_args" => Ok(GeneratedField::ApplicationArgs),
                            "fileUris" | "file_uris" => Ok(GeneratedField::FileUris),
                            "jarUris" | "jar_uris" => Ok(GeneratedField::JarUris),
                            "packages" => Ok(GeneratedField::Packages),
                            "sparkConf" | "spark_conf" => Ok(GeneratedField::SparkConf),
                            "python" => Ok(GeneratedField::Python),
                            "java" => Ok(GeneratedField::Java),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.JobSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut application_file_uri__ = None;
                let mut driver__ = None;
                let mut executor__ = None;
                let mut spark_version__ = None;
                let mut application_args__ = None;
                let mut file_uris__ = None;
                let mut jar_uris__ = None;
                let mut packages__ = None;
                let mut spark_conf__ = None;
                let mut runtime_config__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::ApplicationFileUri => {
                            if application_file_uri__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationFileUri"));
                            }
                            application_file_uri__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Driver => {
                            if driver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driver"));
                            }
                            driver__ = map_.next_value()?;
                        }
                        GeneratedField::Executor => {
                            if executor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executor"));
                            }
                            executor__ = map_.next_value()?;
                        }
                        GeneratedField::SparkVersion => {
                            if spark_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sparkVersion"));
                            }
                            spark_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ApplicationArgs => {
                            if application_args__.is_some() {
                                return Err(serde::de::Error::duplicate_field("applicationArgs"));
                            }
                            application_args__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileUris => {
                            if file_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileUris"));
                            }
                            file_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JarUris => {
                            if jar_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jarUris"));
                            }
                            jar_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Packages => {
                            if packages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packages"));
                            }
                            packages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SparkConf => {
                            if spark_conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sparkConf"));
                            }
                            spark_conf__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Python => {
                            if runtime_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("python"));
                            }
                            runtime_config__ = map_.next_value::<::std::option::Option<_>>()?.map(job_spec::RuntimeConfig::Python)
;
                        }
                        GeneratedField::Java => {
                            if runtime_config__.is_some() {
                                return Err(serde::de::Error::duplicate_field("java"));
                            }
                            runtime_config__ = map_.next_value::<::std::option::Option<_>>()?.map(job_spec::RuntimeConfig::Java)
;
                        }
                    }
                }
                Ok(JobSpec {
                    description: description__,
                    application_file_uri: application_file_uri__.unwrap_or_default(),
                    driver: driver__,
                    executor: executor__,
                    spark_version: spark_version__.unwrap_or_default(),
                    application_args: application_args__.unwrap_or_default(),
                    file_uris: file_uris__.unwrap_or_default(),
                    jar_uris: jar_uris__.unwrap_or_default(),
                    packages: packages__.unwrap_or_default(),
                    spark_conf: spark_conf__.unwrap_or_default(),
                    runtime_config: runtime_config__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.JobSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for JobStatus {
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
        if self.driver_endpoint.is_some() {
            len += 1;
        }
        if self.driver_preset_details.is_some() {
            len += 1;
        }
        if self.executor_preset_details.is_some() {
            len += 1;
        }
        if self.result_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.JobStatus", len)?;
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
        if let Some(v) = self.driver_endpoint.as_ref() {
            struct_ser.serialize_field("driverEndpoint", v)?;
        }
        if let Some(v) = self.driver_preset_details.as_ref() {
            struct_ser.serialize_field("driverPresetDetails", v)?;
        }
        if let Some(v) = self.executor_preset_details.as_ref() {
            struct_ser.serialize_field("executorPresetDetails", v)?;
        }
        if let Some(v) = self.result_details.as_ref() {
            struct_ser.serialize_field("resultDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for JobStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phase",
            "state",
            "driver_endpoint",
            "driverEndpoint",
            "driver_preset_details",
            "driverPresetDetails",
            "executor_preset_details",
            "executorPresetDetails",
            "result_details",
            "resultDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            State,
            DriverEndpoint,
            DriverPresetDetails,
            ExecutorPresetDetails,
            ResultDetails,
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
                            "driverEndpoint" | "driver_endpoint" => Ok(GeneratedField::DriverEndpoint),
                            "driverPresetDetails" | "driver_preset_details" => Ok(GeneratedField::DriverPresetDetails),
                            "executorPresetDetails" | "executor_preset_details" => Ok(GeneratedField::ExecutorPresetDetails),
                            "resultDetails" | "result_details" => Ok(GeneratedField::ResultDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = JobStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.JobStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<JobStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut state__ = None;
                let mut driver_endpoint__ = None;
                let mut driver_preset_details__ = None;
                let mut executor_preset_details__ = None;
                let mut result_details__ = None;
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
                        GeneratedField::DriverEndpoint => {
                            if driver_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverEndpoint"));
                            }
                            driver_endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::DriverPresetDetails => {
                            if driver_preset_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverPresetDetails"));
                            }
                            driver_preset_details__ = map_.next_value()?;
                        }
                        GeneratedField::ExecutorPresetDetails => {
                            if executor_preset_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorPresetDetails"));
                            }
                            executor_preset_details__ = map_.next_value()?;
                        }
                        GeneratedField::ResultDetails => {
                            if result_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultDetails"));
                            }
                            result_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(JobStatus {
                    phase: phase__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    driver_endpoint: driver_endpoint__,
                    driver_preset_details: driver_preset_details__,
                    executor_preset_details: executor_preset_details__,
                    result_details: result_details__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.JobStatus", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Limits {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cpu != 0 {
            len += 1;
        }
        if self.memory_gibibytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.Limits", len)?;
        if self.cpu != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("cpu", ToString::to_string(&self.cpu).as_str())?;
        }
        if self.memory_gibibytes != 0 {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("memoryGibibytes", ToString::to_string(&self.memory_gibibytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Limits {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cpu",
            "memory_gibibytes",
            "memoryGibibytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cpu,
            MemoryGibibytes,
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
                            "cpu" => Ok(GeneratedField::Cpu),
                            "memoryGibibytes" | "memory_gibibytes" => Ok(GeneratedField::MemoryGibibytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Limits;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.Limits")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Limits, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cpu__ = None;
                let mut memory_gibibytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Cpu => {
                            if cpu__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpu"));
                            }
                            cpu__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::MemoryGibibytes => {
                            if memory_gibibytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("memoryGibibytes"));
                            }
                            memory_gibibytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Limits {
                    cpu: cpu__.unwrap_or_default(),
                    memory_gibibytes: memory_gibibytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.Limits", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListClustersRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListClustersRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListClustersRequest", FIELDS, GeneratedVisitor)
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
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListClustersResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListClustersResponse")
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
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListClustersResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListClustersResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListJobsRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListJobsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListJobsRequest {
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
            type Value = ListJobsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListJobsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListJobsRequest, V::Error>
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
                Ok(ListJobsRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListJobsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListJobsResponse {
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
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListJobsResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListJobsResponse {
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
            type Value = ListJobsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListJobsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListJobsResponse, V::Error>
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
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListJobsResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListJobsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSessionsRequest {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListSessionsRequest", len)?;
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
impl<'de> serde::Deserialize<'de> for ListSessionsRequest {
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
            type Value = ListSessionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListSessionsRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSessionsRequest, V::Error>
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
                Ok(ListSessionsRequest {
                    parent_id: parent_id__.unwrap_or_default(),
                    page_size: page_size__.unwrap_or_default(),
                    page_token: page_token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListSessionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ListSessionsResponse {
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
        if self.next_page_token.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.ListSessionsResponse", len)?;
        if !self.items.is_empty() {
            struct_ser.serialize_field("items", &self.items)?;
        }
        if let Some(v) = self.next_page_token.as_ref() {
            struct_ser.serialize_field("nextPageToken", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ListSessionsResponse {
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
            type Value = ListSessionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.ListSessionsResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ListSessionsResponse, V::Error>
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
                            next_page_token__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ListSessionsResponse {
                    items: items__.unwrap_or_default(),
                    next_page_token: next_page_token__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.ListSessionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Password {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.password.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.Password", len)?;
        if !self.password.is_empty() {
            struct_ser.serialize_field("password", &self.password)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Password {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "password",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Password,
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
                            "password" => Ok(GeneratedField::Password),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Password;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.Password")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Password, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut password__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Password => {
                            if password__.is_some() {
                                return Err(serde::de::Error::duplicate_field("password"));
                            }
                            password__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Password {
                    password: password__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.Password", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PythonConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requirements.is_empty() {
            len += 1;
        }
        if !self.file_uris.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.PythonConfig", len)?;
        if !self.requirements.is_empty() {
            struct_ser.serialize_field("requirements", &self.requirements)?;
        }
        if !self.file_uris.is_empty() {
            struct_ser.serialize_field("fileUris", &self.file_uris)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PythonConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requirements",
            "file_uris",
            "fileUris",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requirements,
            FileUris,
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
                            "requirements" => Ok(GeneratedField::Requirements),
                            "fileUris" | "file_uris" => Ok(GeneratedField::FileUris),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PythonConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.PythonConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PythonConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requirements__ = None;
                let mut file_uris__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requirements => {
                            if requirements__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requirements"));
                            }
                            requirements__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileUris => {
                            if file_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileUris"));
                            }
                            file_uris__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PythonConfig {
                    requirements: requirements__.unwrap_or_default(),
                    file_uris: file_uris__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.PythonConfig", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Session {
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.Session", len)?;
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
impl<'de> serde::Deserialize<'de> for Session {
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
            type Value = Session;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.Session")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Session, V::Error>
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
                Ok(Session {
                    metadata: metadata__,
                    spec: spec__,
                    status: status__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.Session", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionSpec {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        if self.driver.is_some() {
            len += 1;
        }
        if self.executor.is_some() {
            len += 1;
        }
        if !self.spark_version.is_empty() {
            len += 1;
        }
        if !self.file_uris.is_empty() {
            len += 1;
        }
        if !self.jar_uris.is_empty() {
            len += 1;
        }
        if !self.packages.is_empty() {
            len += 1;
        }
        if !self.spark_conf.is_empty() {
            len += 1;
        }
        if self.python.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.SessionSpec", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.driver.as_ref() {
            struct_ser.serialize_field("driver", v)?;
        }
        if let Some(v) = self.executor.as_ref() {
            struct_ser.serialize_field("executor", v)?;
        }
        if !self.spark_version.is_empty() {
            struct_ser.serialize_field("sparkVersion", &self.spark_version)?;
        }
        if !self.file_uris.is_empty() {
            struct_ser.serialize_field("fileUris", &self.file_uris)?;
        }
        if !self.jar_uris.is_empty() {
            struct_ser.serialize_field("jarUris", &self.jar_uris)?;
        }
        if !self.packages.is_empty() {
            struct_ser.serialize_field("packages", &self.packages)?;
        }
        if !self.spark_conf.is_empty() {
            struct_ser.serialize_field("sparkConf", &self.spark_conf)?;
        }
        if let Some(v) = self.python.as_ref() {
            struct_ser.serialize_field("python", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SessionSpec {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "driver",
            "executor",
            "spark_version",
            "sparkVersion",
            "file_uris",
            "fileUris",
            "jar_uris",
            "jarUris",
            "packages",
            "spark_conf",
            "sparkConf",
            "python",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Driver,
            Executor,
            SparkVersion,
            FileUris,
            JarUris,
            Packages,
            SparkConf,
            Python,
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
                            "driver" => Ok(GeneratedField::Driver),
                            "executor" => Ok(GeneratedField::Executor),
                            "sparkVersion" | "spark_version" => Ok(GeneratedField::SparkVersion),
                            "fileUris" | "file_uris" => Ok(GeneratedField::FileUris),
                            "jarUris" | "jar_uris" => Ok(GeneratedField::JarUris),
                            "packages" => Ok(GeneratedField::Packages),
                            "sparkConf" | "spark_conf" => Ok(GeneratedField::SparkConf),
                            "python" => Ok(GeneratedField::Python),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionSpec;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.SessionSpec")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SessionSpec, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut driver__ = None;
                let mut executor__ = None;
                let mut spark_version__ = None;
                let mut file_uris__ = None;
                let mut jar_uris__ = None;
                let mut packages__ = None;
                let mut spark_conf__ = None;
                let mut python__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = map_.next_value()?;
                        }
                        GeneratedField::Driver => {
                            if driver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driver"));
                            }
                            driver__ = map_.next_value()?;
                        }
                        GeneratedField::Executor => {
                            if executor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executor"));
                            }
                            executor__ = map_.next_value()?;
                        }
                        GeneratedField::SparkVersion => {
                            if spark_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sparkVersion"));
                            }
                            spark_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::FileUris => {
                            if file_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("fileUris"));
                            }
                            file_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::JarUris => {
                            if jar_uris__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jarUris"));
                            }
                            jar_uris__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Packages => {
                            if packages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("packages"));
                            }
                            packages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::SparkConf => {
                            if spark_conf__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sparkConf"));
                            }
                            spark_conf__ = Some(
                                map_.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                        GeneratedField::Python => {
                            if python__.is_some() {
                                return Err(serde::de::Error::duplicate_field("python"));
                            }
                            python__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SessionSpec {
                    description: description__,
                    driver: driver__,
                    executor: executor__,
                    spark_version: spark_version__.unwrap_or_default(),
                    file_uris: file_uris__.unwrap_or_default(),
                    jar_uris: jar_uris__.unwrap_or_default(),
                    packages: packages__.unwrap_or_default(),
                    spark_conf: spark_conf__.unwrap_or_default(),
                    python: python__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.SessionSpec", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SessionStatus {
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
        if self.spark_connect_endpoint.is_some() {
            len += 1;
        }
        if self.driver_preset_details.is_some() {
            len += 1;
        }
        if self.executor_preset_details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.SessionStatus", len)?;
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
        if let Some(v) = self.spark_connect_endpoint.as_ref() {
            struct_ser.serialize_field("sparkConnectEndpoint", v)?;
        }
        if let Some(v) = self.driver_preset_details.as_ref() {
            struct_ser.serialize_field("driverPresetDetails", v)?;
        }
        if let Some(v) = self.executor_preset_details.as_ref() {
            struct_ser.serialize_field("executorPresetDetails", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SessionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "phase",
            "state",
            "spark_connect_endpoint",
            "sparkConnectEndpoint",
            "driver_preset_details",
            "driverPresetDetails",
            "executor_preset_details",
            "executorPresetDetails",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Phase,
            State,
            SparkConnectEndpoint,
            DriverPresetDetails,
            ExecutorPresetDetails,
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
                            "sparkConnectEndpoint" | "spark_connect_endpoint" => Ok(GeneratedField::SparkConnectEndpoint),
                            "driverPresetDetails" | "driver_preset_details" => Ok(GeneratedField::DriverPresetDetails),
                            "executorPresetDetails" | "executor_preset_details" => Ok(GeneratedField::ExecutorPresetDetails),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SessionStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.msp.spark.v1alpha1.SessionStatus")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<SessionStatus, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut phase__ = None;
                let mut state__ = None;
                let mut spark_connect_endpoint__ = None;
                let mut driver_preset_details__ = None;
                let mut executor_preset_details__ = None;
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
                        GeneratedField::SparkConnectEndpoint => {
                            if spark_connect_endpoint__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sparkConnectEndpoint"));
                            }
                            spark_connect_endpoint__ = map_.next_value()?;
                        }
                        GeneratedField::DriverPresetDetails => {
                            if driver_preset_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("driverPresetDetails"));
                            }
                            driver_preset_details__ = map_.next_value()?;
                        }
                        GeneratedField::ExecutorPresetDetails => {
                            if executor_preset_details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorPresetDetails"));
                            }
                            executor_preset_details__ = map_.next_value()?;
                        }
                    }
                }
                Ok(SessionStatus {
                    phase: phase__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    spark_connect_endpoint: spark_connect_endpoint__,
                    driver_preset_details: driver_preset_details__,
                    executor_preset_details: executor_preset_details__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.SessionStatus", FIELDS, GeneratedVisitor)
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
        let mut struct_ser = serializer.serialize_struct("nebius.msp.spark.v1alpha1.UpdateClusterRequest", len)?;
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
                formatter.write_str("struct nebius.msp.spark.v1alpha1.UpdateClusterRequest")
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
        deserializer.deserialize_struct("nebius.msp.spark.v1alpha1.UpdateClusterRequest", FIELDS, GeneratedVisitor)
    }
}

// @generated
impl serde::Serialize for BadRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.BadRequest", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BadRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
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
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BadRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.BadRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BadRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BadRequest {
                    violations: violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.BadRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for bad_request::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.field.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.BadRequest.Violation", len)?;
        if !self.field.is_empty() {
            struct_ser.serialize_field("field", &self.field)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for bad_request::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "field",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Field,
            Message,
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
                            "field" => Ok(GeneratedField::Field),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = bad_request::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.BadRequest.Violation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<bad_request::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut field__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Field => {
                            if field__.is_some() {
                                return Err(serde::de::Error::duplicate_field("field"));
                            }
                            field__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(bad_request::Violation {
                    field: field__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.BadRequest.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BadResourceState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.BadResourceState", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BadResourceState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
            Message,
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
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BadResourceState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.BadResourceState")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<BadResourceState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(BadResourceState {
                    resource_id: resource_id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.BadResourceState", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for InternalError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.request_id.is_empty() {
            len += 1;
        }
        if !self.trace_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.InternalError", len)?;
        if !self.request_id.is_empty() {
            struct_ser.serialize_field("requestId", &self.request_id)?;
        }
        if !self.trace_id.is_empty() {
            struct_ser.serialize_field("traceId", &self.trace_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for InternalError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "request_id",
            "requestId",
            "trace_id",
            "traceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RequestId,
            TraceId,
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
                            "requestId" | "request_id" => Ok(GeneratedField::RequestId),
                            "traceId" | "trace_id" => Ok(GeneratedField::TraceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = InternalError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.InternalError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<InternalError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut request_id__ = None;
                let mut trace_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::RequestId => {
                            if request_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requestId"));
                            }
                            request_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::TraceId => {
                            if trace_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("traceId"));
                            }
                            trace_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(InternalError {
                    request_id: request_id__.unwrap_or_default(),
                    trace_id: trace_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.InternalError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OperationAborted {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operation_id.is_empty() {
            len += 1;
        }
        if !self.aborted_by_operation_id.is_empty() {
            len += 1;
        }
        if !self.resource_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.OperationAborted", len)?;
        if !self.operation_id.is_empty() {
            struct_ser.serialize_field("operationId", &self.operation_id)?;
        }
        if !self.aborted_by_operation_id.is_empty() {
            struct_ser.serialize_field("abortedByOperationId", &self.aborted_by_operation_id)?;
        }
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OperationAborted {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operation_id",
            "operationId",
            "aborted_by_operation_id",
            "abortedByOperationId",
            "resource_id",
            "resourceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperationId,
            AbortedByOperationId,
            ResourceId,
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
                            "operationId" | "operation_id" => Ok(GeneratedField::OperationId),
                            "abortedByOperationId" | "aborted_by_operation_id" => Ok(GeneratedField::AbortedByOperationId),
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OperationAborted;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.OperationAborted")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OperationAborted, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut operation_id__ = None;
                let mut aborted_by_operation_id__ = None;
                let mut resource_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::OperationId => {
                            if operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationId"));
                            }
                            operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::AbortedByOperationId => {
                            if aborted_by_operation_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("abortedByOperationId"));
                            }
                            aborted_by_operation_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OperationAborted {
                    operation_id: operation_id__.unwrap_or_default(),
                    aborted_by_operation_id: aborted_by_operation_id__.unwrap_or_default(),
                    resource_id: resource_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.OperationAborted", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OutOfRange {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.requested.is_empty() {
            len += 1;
        }
        if !self.limit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.OutOfRange", len)?;
        if !self.requested.is_empty() {
            struct_ser.serialize_field("requested", &self.requested)?;
        }
        if !self.limit.is_empty() {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OutOfRange {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "requested",
            "limit",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Requested,
            Limit,
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
                            "requested" => Ok(GeneratedField::Requested),
                            "limit" => Ok(GeneratedField::Limit),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OutOfRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.OutOfRange")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OutOfRange, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut requested__ = None;
                let mut limit__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Requested => {
                            if requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requested"));
                            }
                            requested__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OutOfRange {
                    requested: requested__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.OutOfRange", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionDenied {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.PermissionDenied", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionDenied {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
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
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionDenied;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.PermissionDenied")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<PermissionDenied, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(PermissionDenied {
                    resource_id: resource_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.PermissionDenied", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuotaFailure {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.QuotaFailure", len)?;
        if !self.violations.is_empty() {
            struct_ser.serialize_field("violations", &self.violations)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuotaFailure {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violations,
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
                            "violations" => Ok(GeneratedField::Violations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuotaFailure;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.QuotaFailure")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<QuotaFailure, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violations__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violations => {
                            if violations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violations"));
                            }
                            violations__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(QuotaFailure {
                    violations: violations__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.QuotaFailure", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for quota_failure::Violation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.quota.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        if !self.limit.is_empty() {
            len += 1;
        }
        if !self.requested.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.QuotaFailure.Violation", len)?;
        if !self.quota.is_empty() {
            struct_ser.serialize_field("quota", &self.quota)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        if !self.limit.is_empty() {
            struct_ser.serialize_field("limit", &self.limit)?;
        }
        if !self.requested.is_empty() {
            struct_ser.serialize_field("requested", &self.requested)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for quota_failure::Violation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "quota",
            "message",
            "limit",
            "requested",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Quota,
            Message,
            Limit,
            Requested,
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
                            "quota" => Ok(GeneratedField::Quota),
                            "message" => Ok(GeneratedField::Message),
                            "limit" => Ok(GeneratedField::Limit),
                            "requested" => Ok(GeneratedField::Requested),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = quota_failure::Violation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.QuotaFailure.Violation")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<quota_failure::Violation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut quota__ = None;
                let mut message__ = None;
                let mut limit__ = None;
                let mut requested__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Quota => {
                            if quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quota"));
                            }
                            quota__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Limit => {
                            if limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("limit"));
                            }
                            limit__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Requested => {
                            if requested__.is_some() {
                                return Err(serde::de::Error::duplicate_field("requested"));
                            }
                            requested__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(quota_failure::Violation {
                    quota: quota__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                    limit: limit__.unwrap_or_default(),
                    requested: requested__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.QuotaFailure.Violation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceAlreadyExists {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.ResourceAlreadyExists", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceAlreadyExists {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
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
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceAlreadyExists;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.ResourceAlreadyExists")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceAlreadyExists, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceAlreadyExists {
                    resource_id: resource_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.ResourceAlreadyExists", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceConflict {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        if !self.message.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.ResourceConflict", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        if !self.message.is_empty() {
            struct_ser.serialize_field("message", &self.message)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceConflict {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
            "message",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
            Message,
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
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            "message" => Ok(GeneratedField::Message),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceConflict;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.ResourceConflict")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceConflict, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                let mut message__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Message => {
                            if message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("message"));
                            }
                            message__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceConflict {
                    resource_id: resource_id__.unwrap_or_default(),
                    message: message__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.ResourceConflict", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ResourceNotFound {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.resource_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.ResourceNotFound", len)?;
        if !self.resource_id.is_empty() {
            struct_ser.serialize_field("resourceId", &self.resource_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ResourceNotFound {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "resource_id",
            "resourceId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ResourceId,
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
                            "resourceId" | "resource_id" => Ok(GeneratedField::ResourceId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ResourceNotFound;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.ResourceNotFound")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ResourceNotFound, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut resource_id__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ResourceId => {
                            if resource_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceId"));
                            }
                            resource_id__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ResourceNotFound {
                    resource_id: resource_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.ResourceNotFound", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceError {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.service.is_empty() {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if self.retry_type != 0 {
            len += 1;
        }
        if self.details.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.ServiceError", len)?;
        if !self.service.is_empty() {
            struct_ser.serialize_field("service", &self.service)?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", &self.code)?;
        }
        if self.retry_type != 0 {
            let v = service_error::RetryType::try_from(self.retry_type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.retry_type)))?;
            struct_ser.serialize_field("retryType", &v)?;
        }
        if let Some(v) = self.details.as_ref() {
            match v {
                service_error::Details::BadRequest(v) => {
                    struct_ser.serialize_field("badRequest", v)?;
                }
                service_error::Details::BadResourceState(v) => {
                    struct_ser.serialize_field("badResourceState", v)?;
                }
                service_error::Details::ResourceNotFound(v) => {
                    struct_ser.serialize_field("resourceNotFound", v)?;
                }
                service_error::Details::ResourceAlreadyExists(v) => {
                    struct_ser.serialize_field("resourceAlreadyExists", v)?;
                }
                service_error::Details::OutOfRange(v) => {
                    struct_ser.serialize_field("outOfRange", v)?;
                }
                service_error::Details::PermissionDenied(v) => {
                    struct_ser.serialize_field("permissionDenied", v)?;
                }
                service_error::Details::ResourceConflict(v) => {
                    struct_ser.serialize_field("resourceConflict", v)?;
                }
                service_error::Details::OperationAborted(v) => {
                    struct_ser.serialize_field("operationAborted", v)?;
                }
                service_error::Details::TooManyRequests(v) => {
                    struct_ser.serialize_field("tooManyRequests", v)?;
                }
                service_error::Details::QuotaFailure(v) => {
                    struct_ser.serialize_field("quotaFailure", v)?;
                }
                service_error::Details::InternalError(v) => {
                    struct_ser.serialize_field("internalError", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceError {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "service",
            "code",
            "retry_type",
            "retryType",
            "bad_request",
            "badRequest",
            "bad_resource_state",
            "badResourceState",
            "resource_not_found",
            "resourceNotFound",
            "resource_already_exists",
            "resourceAlreadyExists",
            "out_of_range",
            "outOfRange",
            "permission_denied",
            "permissionDenied",
            "resource_conflict",
            "resourceConflict",
            "operation_aborted",
            "operationAborted",
            "too_many_requests",
            "tooManyRequests",
            "quota_failure",
            "quotaFailure",
            "internal_error",
            "internalError",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Service,
            Code,
            RetryType,
            BadRequest,
            BadResourceState,
            ResourceNotFound,
            ResourceAlreadyExists,
            OutOfRange,
            PermissionDenied,
            ResourceConflict,
            OperationAborted,
            TooManyRequests,
            QuotaFailure,
            InternalError,
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
                            "service" => Ok(GeneratedField::Service),
                            "code" => Ok(GeneratedField::Code),
                            "retryType" | "retry_type" => Ok(GeneratedField::RetryType),
                            "badRequest" | "bad_request" => Ok(GeneratedField::BadRequest),
                            "badResourceState" | "bad_resource_state" => Ok(GeneratedField::BadResourceState),
                            "resourceNotFound" | "resource_not_found" => Ok(GeneratedField::ResourceNotFound),
                            "resourceAlreadyExists" | "resource_already_exists" => Ok(GeneratedField::ResourceAlreadyExists),
                            "outOfRange" | "out_of_range" => Ok(GeneratedField::OutOfRange),
                            "permissionDenied" | "permission_denied" => Ok(GeneratedField::PermissionDenied),
                            "resourceConflict" | "resource_conflict" => Ok(GeneratedField::ResourceConflict),
                            "operationAborted" | "operation_aborted" => Ok(GeneratedField::OperationAborted),
                            "tooManyRequests" | "too_many_requests" => Ok(GeneratedField::TooManyRequests),
                            "quotaFailure" | "quota_failure" => Ok(GeneratedField::QuotaFailure),
                            "internalError" | "internal_error" => Ok(GeneratedField::InternalError),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceError;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.ServiceError")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ServiceError, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut service__ = None;
                let mut code__ = None;
                let mut retry_type__ = None;
                let mut details__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Service => {
                            if service__.is_some() {
                                return Err(serde::de::Error::duplicate_field("service"));
                            }
                            service__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RetryType => {
                            if retry_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("retryType"));
                            }
                            retry_type__ = Some(map_.next_value::<service_error::RetryType>()? as i32);
                        }
                        GeneratedField::BadRequest => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("badRequest"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::BadRequest)
;
                        }
                        GeneratedField::BadResourceState => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("badResourceState"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::BadResourceState)
;
                        }
                        GeneratedField::ResourceNotFound => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceNotFound"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::ResourceNotFound)
;
                        }
                        GeneratedField::ResourceAlreadyExists => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceAlreadyExists"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::ResourceAlreadyExists)
;
                        }
                        GeneratedField::OutOfRange => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("outOfRange"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::OutOfRange)
;
                        }
                        GeneratedField::PermissionDenied => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissionDenied"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::PermissionDenied)
;
                        }
                        GeneratedField::ResourceConflict => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resourceConflict"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::ResourceConflict)
;
                        }
                        GeneratedField::OperationAborted => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operationAborted"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::OperationAborted)
;
                        }
                        GeneratedField::TooManyRequests => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tooManyRequests"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::TooManyRequests)
;
                        }
                        GeneratedField::QuotaFailure => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("quotaFailure"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::QuotaFailure)
;
                        }
                        GeneratedField::InternalError => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("internalError"));
                            }
                            details__ = map_.next_value::<::std::option::Option<_>>()?.map(service_error::Details::InternalError)
;
                        }
                    }
                }
                Ok(ServiceError {
                    service: service__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    retry_type: retry_type__.unwrap_or_default(),
                    details: details__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.ServiceError", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for service_error::RetryType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "UNSPECIFIED",
            Self::Call => "CALL",
            Self::UnitOfWork => "UNIT_OF_WORK",
            Self::Nothing => "NOTHING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for service_error::RetryType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNSPECIFIED",
            "CALL",
            "UNIT_OF_WORK",
            "NOTHING",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = service_error::RetryType;

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
                    "UNSPECIFIED" => Ok(service_error::RetryType::Unspecified),
                    "CALL" => Ok(service_error::RetryType::Call),
                    "UNIT_OF_WORK" => Ok(service_error::RetryType::UnitOfWork),
                    "NOTHING" => Ok(service_error::RetryType::Nothing),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TooManyRequests {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.violation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.common.error.v1alpha1.TooManyRequests", len)?;
        if !self.violation.is_empty() {
            struct_ser.serialize_field("violation", &self.violation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TooManyRequests {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "violation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Violation,
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
                            "violation" => Ok(GeneratedField::Violation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TooManyRequests;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.common.error.v1alpha1.TooManyRequests")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<TooManyRequests, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut violation__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Violation => {
                            if violation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("violation"));
                            }
                            violation__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(TooManyRequests {
                    violation: violation__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.common.error.v1alpha1.TooManyRequests", FIELDS, GeneratedVisitor)
    }
}

// @generated
impl serde::Serialize for Action {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Undefined => "ACTION_UNDEFINED",
            Self::Nop => "NOP",
            Self::Update => "UPDATE",
            Self::Restart => "RESTART",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Action {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "ACTION_UNDEFINED",
            "NOP",
            "UPDATE",
            "RESTART",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Action;

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
                    "ACTION_UNDEFINED" => Ok(Action::Undefined),
                    "NOP" => Ok(Action::Nop),
                    "UPDATE" => Ok(Action::Update),
                    "RESTART" => Ok(Action::Restart),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgentState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::StateUndefined => "STATE_UNDEFINED",
            Self::StateHealthy => "STATE_HEALTHY",
            Self::StateError => "STATE_ERROR",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgentState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "STATE_UNDEFINED",
            "STATE_HEALTHY",
            "STATE_ERROR",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentState;

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
                    "STATE_UNDEFINED" => Ok(AgentState::StateUndefined),
                    "STATE_HEALTHY" => Ok(AgentState::StateHealthy),
                    "STATE_ERROR" => Ok(AgentState::StateError),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for AgentType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::AgentUndefined => "AGENT_UNDEFINED",
            Self::O11yAgent => "O11Y_AGENT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for AgentType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "AGENT_UNDEFINED",
            "O11Y_AGENT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AgentType;

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
                    "AGENT_UNDEFINED" => Ok(AgentType::AgentUndefined),
                    "O11Y_AGENT" => Ok(AgentType::O11yAgent),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GetVersionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if !self.agent_version.is_empty() {
            len += 1;
        }
        if !self.updater_version.is_empty() {
            len += 1;
        }
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if !self.instance_id.is_empty() {
            len += 1;
        }
        if self.os_info.is_some() {
            len += 1;
        }
        if self.agent_state != 0 {
            len += 1;
        }
        if self.agent_uptime.is_some() {
            len += 1;
        }
        if self.system_uptime.is_some() {
            len += 1;
        }
        if self.updater_uptime.is_some() {
            len += 1;
        }
        if !self.agent_state_messages.is_empty() {
            len += 1;
        }
        if !self.last_update_error.is_empty() {
            len += 1;
        }
        if !self.mk8s_cluster_id.is_empty() {
            len += 1;
        }
        if self.modules_health.is_some() {
            len += 1;
        }
        if !self.cloud_init_status.is_empty() {
            len += 1;
        }
        if self.instance_id_used_fallback {
            len += 1;
        }
        if !self.last_agent_logs.is_empty() {
            len += 1;
        }
        if !self.gpu_model.is_empty() {
            len += 1;
        }
        if self.gpu_number != 0 {
            len += 1;
        }
        if !self.dcgm_version.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.GetVersionRequest", len)?;
        if self.r#type != 0 {
            let v = AgentType::try_from(self.r#type)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.r#type)))?;
            struct_ser.serialize_field("type", &v)?;
        }
        if !self.agent_version.is_empty() {
            struct_ser.serialize_field("agentVersion", &self.agent_version)?;
        }
        if !self.updater_version.is_empty() {
            struct_ser.serialize_field("updaterVersion", &self.updater_version)?;
        }
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", &self.parent_id)?;
        }
        if !self.instance_id.is_empty() {
            struct_ser.serialize_field("instanceId", &self.instance_id)?;
        }
        if let Some(v) = self.os_info.as_ref() {
            struct_ser.serialize_field("osInfo", v)?;
        }
        if self.agent_state != 0 {
            let v = AgentState::try_from(self.agent_state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.agent_state)))?;
            struct_ser.serialize_field("agentState", &v)?;
        }
        if let Some(v) = self.agent_uptime.as_ref() {
            struct_ser.serialize_field("agentUptime", v)?;
        }
        if let Some(v) = self.system_uptime.as_ref() {
            struct_ser.serialize_field("systemUptime", v)?;
        }
        if let Some(v) = self.updater_uptime.as_ref() {
            struct_ser.serialize_field("updaterUptime", v)?;
        }
        if !self.agent_state_messages.is_empty() {
            struct_ser.serialize_field("agentStateMessages", &self.agent_state_messages)?;
        }
        if !self.last_update_error.is_empty() {
            struct_ser.serialize_field("lastUpdateError", &self.last_update_error)?;
        }
        if !self.mk8s_cluster_id.is_empty() {
            struct_ser.serialize_field("mk8sClusterId", &self.mk8s_cluster_id)?;
        }
        if let Some(v) = self.modules_health.as_ref() {
            struct_ser.serialize_field("modulesHealth", v)?;
        }
        if !self.cloud_init_status.is_empty() {
            struct_ser.serialize_field("cloudInitStatus", &self.cloud_init_status)?;
        }
        if self.instance_id_used_fallback {
            struct_ser.serialize_field("instanceIdUsedFallback", &self.instance_id_used_fallback)?;
        }
        if !self.last_agent_logs.is_empty() {
            struct_ser.serialize_field("lastAgentLogs", &self.last_agent_logs)?;
        }
        if !self.gpu_model.is_empty() {
            struct_ser.serialize_field("gpuModel", &self.gpu_model)?;
        }
        if self.gpu_number != 0 {
            struct_ser.serialize_field("gpuNumber", &self.gpu_number)?;
        }
        if !self.dcgm_version.is_empty() {
            struct_ser.serialize_field("dcgmVersion", &self.dcgm_version)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "agent_version",
            "agentVersion",
            "updater_version",
            "updaterVersion",
            "parent_id",
            "parentId",
            "instance_id",
            "instanceId",
            "os_info",
            "osInfo",
            "agent_state",
            "agentState",
            "agent_uptime",
            "agentUptime",
            "system_uptime",
            "systemUptime",
            "updater_uptime",
            "updaterUptime",
            "agent_state_messages",
            "agentStateMessages",
            "last_update_error",
            "lastUpdateError",
            "mk8s_cluster_id",
            "mk8sClusterId",
            "modules_health",
            "modulesHealth",
            "cloud_init_status",
            "cloudInitStatus",
            "instance_id_used_fallback",
            "instanceIdUsedFallback",
            "last_agent_logs",
            "lastAgentLogs",
            "gpu_model",
            "gpuModel",
            "gpu_number",
            "gpuNumber",
            "dcgm_version",
            "dcgmVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            AgentVersion,
            UpdaterVersion,
            ParentId,
            InstanceId,
            OsInfo,
            AgentState,
            AgentUptime,
            SystemUptime,
            UpdaterUptime,
            AgentStateMessages,
            LastUpdateError,
            Mk8sClusterId,
            ModulesHealth,
            CloudInitStatus,
            InstanceIdUsedFallback,
            LastAgentLogs,
            GpuModel,
            GpuNumber,
            DcgmVersion,
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
                            "type" => Ok(GeneratedField::Type),
                            "agentVersion" | "agent_version" => Ok(GeneratedField::AgentVersion),
                            "updaterVersion" | "updater_version" => Ok(GeneratedField::UpdaterVersion),
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "instanceId" | "instance_id" => Ok(GeneratedField::InstanceId),
                            "osInfo" | "os_info" => Ok(GeneratedField::OsInfo),
                            "agentState" | "agent_state" => Ok(GeneratedField::AgentState),
                            "agentUptime" | "agent_uptime" => Ok(GeneratedField::AgentUptime),
                            "systemUptime" | "system_uptime" => Ok(GeneratedField::SystemUptime),
                            "updaterUptime" | "updater_uptime" => Ok(GeneratedField::UpdaterUptime),
                            "agentStateMessages" | "agent_state_messages" => Ok(GeneratedField::AgentStateMessages),
                            "lastUpdateError" | "last_update_error" => Ok(GeneratedField::LastUpdateError),
                            "mk8sClusterId" | "mk8s_cluster_id" => Ok(GeneratedField::Mk8sClusterId),
                            "modulesHealth" | "modules_health" => Ok(GeneratedField::ModulesHealth),
                            "cloudInitStatus" | "cloud_init_status" => Ok(GeneratedField::CloudInitStatus),
                            "instanceIdUsedFallback" | "instance_id_used_fallback" => Ok(GeneratedField::InstanceIdUsedFallback),
                            "lastAgentLogs" | "last_agent_logs" => Ok(GeneratedField::LastAgentLogs),
                            "gpuModel" | "gpu_model" => Ok(GeneratedField::GpuModel),
                            "gpuNumber" | "gpu_number" => Ok(GeneratedField::GpuNumber),
                            "dcgmVersion" | "dcgm_version" => Ok(GeneratedField::DcgmVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.GetVersionRequest")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut agent_version__ = None;
                let mut updater_version__ = None;
                let mut parent_id__ = None;
                let mut instance_id__ = None;
                let mut os_info__ = None;
                let mut agent_state__ = None;
                let mut agent_uptime__ = None;
                let mut system_uptime__ = None;
                let mut updater_uptime__ = None;
                let mut agent_state_messages__ = None;
                let mut last_update_error__ = None;
                let mut mk8s_cluster_id__ = None;
                let mut modules_health__ = None;
                let mut cloud_init_status__ = None;
                let mut instance_id_used_fallback__ = None;
                let mut last_agent_logs__ = None;
                let mut gpu_model__ = None;
                let mut gpu_number__ = None;
                let mut dcgm_version__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map_.next_value::<AgentType>()? as i32);
                        }
                        GeneratedField::AgentVersion => {
                            if agent_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentVersion"));
                            }
                            agent_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::UpdaterVersion => {
                            if updater_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updaterVersion"));
                            }
                            updater_version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstanceId => {
                            if instance_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceId"));
                            }
                            instance_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::OsInfo => {
                            if os_info__.is_some() {
                                return Err(serde::de::Error::duplicate_field("osInfo"));
                            }
                            os_info__ = map_.next_value()?;
                        }
                        GeneratedField::AgentState => {
                            if agent_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentState"));
                            }
                            agent_state__ = Some(map_.next_value::<AgentState>()? as i32);
                        }
                        GeneratedField::AgentUptime => {
                            if agent_uptime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentUptime"));
                            }
                            agent_uptime__ = map_.next_value()?;
                        }
                        GeneratedField::SystemUptime => {
                            if system_uptime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("systemUptime"));
                            }
                            system_uptime__ = map_.next_value()?;
                        }
                        GeneratedField::UpdaterUptime => {
                            if updater_uptime__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updaterUptime"));
                            }
                            updater_uptime__ = map_.next_value()?;
                        }
                        GeneratedField::AgentStateMessages => {
                            if agent_state_messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("agentStateMessages"));
                            }
                            agent_state_messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastUpdateError => {
                            if last_update_error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastUpdateError"));
                            }
                            last_update_error__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Mk8sClusterId => {
                            if mk8s_cluster_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mk8sClusterId"));
                            }
                            mk8s_cluster_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ModulesHealth => {
                            if modules_health__.is_some() {
                                return Err(serde::de::Error::duplicate_field("modulesHealth"));
                            }
                            modules_health__ = map_.next_value()?;
                        }
                        GeneratedField::CloudInitStatus => {
                            if cloud_init_status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cloudInitStatus"));
                            }
                            cloud_init_status__ = Some(map_.next_value()?);
                        }
                        GeneratedField::InstanceIdUsedFallback => {
                            if instance_id_used_fallback__.is_some() {
                                return Err(serde::de::Error::duplicate_field("instanceIdUsedFallback"));
                            }
                            instance_id_used_fallback__ = Some(map_.next_value()?);
                        }
                        GeneratedField::LastAgentLogs => {
                            if last_agent_logs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastAgentLogs"));
                            }
                            last_agent_logs__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GpuModel => {
                            if gpu_model__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpuModel"));
                            }
                            gpu_model__ = Some(map_.next_value()?);
                        }
                        GeneratedField::GpuNumber => {
                            if gpu_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpuNumber"));
                            }
                            gpu_number__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::DcgmVersion => {
                            if dcgm_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("dcgmVersion"));
                            }
                            dcgm_version__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(GetVersionRequest {
                    r#type: r#type__.unwrap_or_default(),
                    agent_version: agent_version__.unwrap_or_default(),
                    updater_version: updater_version__.unwrap_or_default(),
                    parent_id: parent_id__.unwrap_or_default(),
                    instance_id: instance_id__.unwrap_or_default(),
                    os_info: os_info__,
                    agent_state: agent_state__.unwrap_or_default(),
                    agent_uptime: agent_uptime__,
                    system_uptime: system_uptime__,
                    updater_uptime: updater_uptime__,
                    agent_state_messages: agent_state_messages__.unwrap_or_default(),
                    last_update_error: last_update_error__.unwrap_or_default(),
                    mk8s_cluster_id: mk8s_cluster_id__.unwrap_or_default(),
                    modules_health: modules_health__,
                    cloud_init_status: cloud_init_status__.unwrap_or_default(),
                    instance_id_used_fallback: instance_id_used_fallback__.unwrap_or_default(),
                    last_agent_logs: last_agent_logs__.unwrap_or_default(),
                    gpu_model: gpu_model__.unwrap_or_default(),
                    gpu_number: gpu_number__.unwrap_or_default(),
                    dcgm_version: dcgm_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.GetVersionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetVersionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.action != 0 {
            len += 1;
        }
        if self.response.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.GetVersionResponse", len)?;
        if self.action != 0 {
            let v = Action::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if let Some(v) = self.response.as_ref() {
            match v {
                get_version_response::Response::Nop(v) => {
                    struct_ser.serialize_field("nop", v)?;
                }
                get_version_response::Response::Update(v) => {
                    struct_ser.serialize_field("update", v)?;
                }
                get_version_response::Response::Restart(v) => {
                    struct_ser.serialize_field("restart", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetVersionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "action",
            "nop",
            "update",
            "restart",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Action,
            Nop,
            Update,
            Restart,
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
                            "action" => Ok(GeneratedField::Action),
                            "nop" => Ok(GeneratedField::Nop),
                            "update" => Ok(GeneratedField::Update),
                            "restart" => Ok(GeneratedField::Restart),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetVersionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.GetVersionResponse")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<GetVersionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut action__ = None;
                let mut response__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<Action>()? as i32);
                        }
                        GeneratedField::Nop => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nop"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_version_response::Response::Nop)
;
                        }
                        GeneratedField::Update => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_version_response::Response::Update)
;
                        }
                        GeneratedField::Restart => {
                            if response__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restart"));
                            }
                            response__ = map_.next_value::<::std::option::Option<_>>()?.map(get_version_response::Response::Restart)
;
                        }
                    }
                }
                Ok(GetVersionResponse {
                    action: action__.unwrap_or_default(),
                    response: response__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.GetVersionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModuleHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.state != 0 {
            len += 1;
        }
        if !self.messages.is_empty() {
            len += 1;
        }
        if !self.parameters.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.ModuleHealth", len)?;
        if self.state != 0 {
            let v = AgentState::try_from(self.state)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.state)))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.messages.is_empty() {
            struct_ser.serialize_field("messages", &self.messages)?;
        }
        if !self.parameters.is_empty() {
            struct_ser.serialize_field("parameters", &self.parameters)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModuleHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "state",
            "messages",
            "parameters",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            State,
            Messages,
            Parameters,
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
                            "state" => Ok(GeneratedField::State),
                            "messages" => Ok(GeneratedField::Messages),
                            "parameters" => Ok(GeneratedField::Parameters),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModuleHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.ModuleHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModuleHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut state__ = None;
                let mut messages__ = None;
                let mut parameters__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(map_.next_value::<AgentState>()? as i32);
                        }
                        GeneratedField::Messages => {
                            if messages__.is_some() {
                                return Err(serde::de::Error::duplicate_field("messages"));
                            }
                            messages__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Parameters => {
                            if parameters__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parameters"));
                            }
                            parameters__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(ModuleHealth {
                    state: state__.unwrap_or_default(),
                    messages: messages__.unwrap_or_default(),
                    parameters: parameters__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.ModuleHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ModulesHealth {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.process.is_some() {
            len += 1;
        }
        if self.gpu_pipeline.is_some() {
            len += 1;
        }
        if self.cpu_pipeline.is_some() {
            len += 1;
        }
        if self.cilium_pipeline.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.ModulesHealth", len)?;
        if let Some(v) = self.process.as_ref() {
            struct_ser.serialize_field("process", v)?;
        }
        if let Some(v) = self.gpu_pipeline.as_ref() {
            struct_ser.serialize_field("gpuPipeline", v)?;
        }
        if let Some(v) = self.cpu_pipeline.as_ref() {
            struct_ser.serialize_field("cpuPipeline", v)?;
        }
        if let Some(v) = self.cilium_pipeline.as_ref() {
            struct_ser.serialize_field("ciliumPipeline", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ModulesHealth {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "process",
            "gpu_pipeline",
            "gpuPipeline",
            "cpu_pipeline",
            "cpuPipeline",
            "cilium_pipeline",
            "ciliumPipeline",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Process,
            GpuPipeline,
            CpuPipeline,
            CiliumPipeline,
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
                            "process" => Ok(GeneratedField::Process),
                            "gpuPipeline" | "gpu_pipeline" => Ok(GeneratedField::GpuPipeline),
                            "cpuPipeline" | "cpu_pipeline" => Ok(GeneratedField::CpuPipeline),
                            "ciliumPipeline" | "cilium_pipeline" => Ok(GeneratedField::CiliumPipeline),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ModulesHealth;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.ModulesHealth")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ModulesHealth, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut process__ = None;
                let mut gpu_pipeline__ = None;
                let mut cpu_pipeline__ = None;
                let mut cilium_pipeline__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Process => {
                            if process__.is_some() {
                                return Err(serde::de::Error::duplicate_field("process"));
                            }
                            process__ = map_.next_value()?;
                        }
                        GeneratedField::GpuPipeline => {
                            if gpu_pipeline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gpuPipeline"));
                            }
                            gpu_pipeline__ = map_.next_value()?;
                        }
                        GeneratedField::CpuPipeline => {
                            if cpu_pipeline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuPipeline"));
                            }
                            cpu_pipeline__ = map_.next_value()?;
                        }
                        GeneratedField::CiliumPipeline => {
                            if cilium_pipeline__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ciliumPipeline"));
                            }
                            cilium_pipeline__ = map_.next_value()?;
                        }
                    }
                }
                Ok(ModulesHealth {
                    process: process__,
                    gpu_pipeline: gpu_pipeline__,
                    cpu_pipeline: cpu_pipeline__,
                    cilium_pipeline: cilium_pipeline__,
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.ModulesHealth", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NopActionParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.NopActionParams", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NopActionParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NopActionParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.NopActionParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<NopActionParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(NopActionParams {
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.NopActionParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for OsInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.uname.is_empty() {
            len += 1;
        }
        if !self.architecture.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.OSInfo", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.uname.is_empty() {
            struct_ser.serialize_field("uname", &self.uname)?;
        }
        if !self.architecture.is_empty() {
            struct_ser.serialize_field("architecture", &self.architecture)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for OsInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "uname",
            "architecture",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Uname,
            Architecture,
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
                            "name" => Ok(GeneratedField::Name),
                            "uname" => Ok(GeneratedField::Uname),
                            "architecture" => Ok(GeneratedField::Architecture),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = OsInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.OSInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<OsInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut uname__ = None;
                let mut architecture__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Uname => {
                            if uname__.is_some() {
                                return Err(serde::de::Error::duplicate_field("uname"));
                            }
                            uname__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Architecture => {
                            if architecture__.is_some() {
                                return Err(serde::de::Error::duplicate_field("architecture"));
                            }
                            architecture__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(OsInfo {
                    name: name__.unwrap_or_default(),
                    uname: uname__.unwrap_or_default(),
                    architecture: architecture__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.OSInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Parameter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.Parameter", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Parameter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Value,
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
                            "name" => Ok(GeneratedField::Name),
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Parameter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.Parameter")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Parameter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut value__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(Parameter {
                    name: name__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.Parameter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RestartActionParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.RestartActionParams", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RestartActionParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RestartActionParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.RestartActionParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RestartActionParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map_.next_key::<GeneratedField>()?.is_some() {
                    let _ = map_.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(RestartActionParams {
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.RestartActionParams", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UpdateActionParams {
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
        if !self.repo_url.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("nebius.logging.agentmanager.v1.UpdateActionParams", len)?;
        if !self.version.is_empty() {
            struct_ser.serialize_field("version", &self.version)?;
        }
        if !self.repo_url.is_empty() {
            struct_ser.serialize_field("repoUrl", &self.repo_url)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UpdateActionParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "version",
            "repo_url",
            "repoUrl",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Version,
            RepoUrl,
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
                            "repoUrl" | "repo_url" => Ok(GeneratedField::RepoUrl),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UpdateActionParams;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct nebius.logging.agentmanager.v1.UpdateActionParams")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<UpdateActionParams, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut version__ = None;
                let mut repo_url__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Version => {
                            if version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            version__ = Some(map_.next_value()?);
                        }
                        GeneratedField::RepoUrl => {
                            if repo_url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("repoUrl"));
                            }
                            repo_url__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(UpdateActionParams {
                    version: version__.unwrap_or_default(),
                    repo_url: repo_url__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("nebius.logging.agentmanager.v1.UpdateActionParams", FIELDS, GeneratedVisitor)
    }
}

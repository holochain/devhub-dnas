mod validation;

pub use apphub_types;
pub use apphub_types::*;
pub use hc_crud;

use serde::{
    Deserialize, Deserializer,
};
use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
    scoped_type_connector,
    ScopedTypeConnector,
};
use hc_crud::{
    entry_model,
};



/// The entry types defined for this integrity app
#[hdk_entry_types]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    #[entry_type]
    App(AppEntry),
    Ui(UiEntry),
    WebApp(WebAppEntry),
    WebAppPackage(WebAppPackageEntry),
    WebAppPackageVersion(WebAppPackageVersionEntry),
}

scoped_type_connector!(
    EntryTypesUnit::App,
    EntryTypes::App( AppEntry )
);
scoped_type_connector!(
    EntryTypesUnit::Ui,
    EntryTypes::Ui( UiEntry )
);
scoped_type_connector!(
    EntryTypesUnit::WebApp,
    EntryTypes::WebApp( WebAppEntry )
);
scoped_type_connector!(
    EntryTypesUnit::WebAppPackage,
    EntryTypes::WebAppPackage( WebAppPackageEntry )
);
scoped_type_connector!(
    EntryTypesUnit::WebAppPackageVersion,
    EntryTypes::WebAppPackageVersion( WebAppPackageVersionEntry )
);

// Entity implementations
entry_model!( EntryTypes::App( AppEntry ) );
entry_model!( EntryTypes::Ui( UiEntry ) );
entry_model!( EntryTypes::WebApp( WebAppEntry ) );
entry_model!( EntryTypes::WebAppPackage( WebAppPackageEntry ) );
entry_model!( EntryTypes::WebAppPackageVersion( WebAppPackageVersionEntry ) );



/// The link types defined for this integrity app
#[hdk_link_types]
pub enum LinkTypes {
    App,
    Ui,
    WebApp,
    WebAppPackage,
    WebAppPackageVersion,

    AgentToApp,
    AgentToUi,
    AgentToWebApp,
    AgentToWebAppPackage,
    AgentToWebAppPackageVersion,

    WebAppPackageToWebAppPackageVersion,
}

impl TryFrom<String> for LinkTypes {
    type Error = WasmError;

    fn try_from(name: String) -> Result<Self, Self::Error> {
        Ok(
            match name.as_str() {
                "App" => LinkTypes::App,
                "Ui" => LinkTypes::Ui,
                "WebApp" => LinkTypes::WebApp,
                "WebAppPackage" => LinkTypes::WebAppPackage,
                "WebAppPackageVersion" => LinkTypes::WebAppPackageVersion,

                "AgentToApp" => LinkTypes::AgentToApp,
                "AgentToUi" => LinkTypes::AgentToUi,
                "AgentToWebApp" => LinkTypes::AgentToWebApp,
                "AgentToWebAppPackage" => LinkTypes::AgentToWebAppPackage,
                "AgentToWebAppPackageVersion" => LinkTypes::AgentToWebAppPackageVersion,

                "WebAppPackageToWebAppPackageVersion" => LinkTypes::WebAppPackageToWebAppPackageVersion,

                _ => return Err(guest_error!(format!("Unknown LinkTypes variant: {}", name ))),
            }
        )
    }
}

impl<'de> Deserialize<'de> for LinkTypes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Ok(
            LinkTypes::try_from( s.clone() )
                .or(Err(serde::de::Error::custom(format!("Unknown LinkTypes variant: {}", s))))?
        )
    }
}

use objc::runtime::Object;
use objc::{class, msg_send, sel, sel_impl};
use objc_id::Id;

use crate::foundation::{id, ClassMap, NSString};
use crate::uikit::scene::SessionRole;

/// A wrapper for UISceneConfiguration.
///
/// Due to the way we have to implement this, you likely never need to touch this.
#[derive(Debug)]
pub struct SceneConfig(Id<Object>);

impl SceneConfig {
    /// Creates a new `UISceneConfiguration` with the specified name and session role, retains it,
    /// and returns it.
    pub fn new(name: &str, role: SessionRole) -> Self {
        let delegate_class = ClassMap::static_load("RSTWindowSceneDelegate", Some("UIResponder"))
            .expect("A crucial iOS step was missed - the scene delegate class is either not loaded or misnamed");

        SceneConfig(unsafe {
            let name = NSString::new(name);
            let role = NSString::from(role);

            let cls = class!(UISceneConfiguration);
            let config: id = msg_send![cls, configurationWithName:name sessionRole:role];

            let _: () = msg_send![config, setSceneClass: class!(UIWindowScene)];
            let _: () = msg_send![config, setDelegateClass: delegate_class];

            Id::from_ptr(config)
        })
    }

    /// Consumes and returns the underlying `UISceneConfiguration`.
    pub fn into_inner(mut self) -> id {
        &mut *self.0
    }
}

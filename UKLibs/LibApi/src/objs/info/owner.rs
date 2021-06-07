/*! `Object` owner user */

use crate::{
    bits::obj::grants::Grants,
    ents::impls::{
        group::OSGroup,
        user::OSUser
    },
    objs::object::Object
};

/**
 * Stores the `Object` owner(s) with the protection `Grants`
 */
#[derive(Debug, Default, Copy, Clone)]
pub struct ObjOwner<T>
    where T: Object {
    m_os_user: OSUser,
    m_os_group: OSGroup,
    m_protection: Grants<T>
}

#[cfg(feature = "enable_kernel_methods")]
impl<T> ObjOwner<T> where T: Object {
    /**
     * Constructs a `ObjUser` with the given parameters
     */
    pub const fn new(os_user: OSUser, os_group: OSGroup, protection: Grants<T>) -> Self {
        Self { m_os_user: os_user,
               m_os_group: os_group,
               m_protection: protection }
    }
}

impl<T> ObjOwner<T> where T: Object {
    /**
     * Returns the owner `OSUser` reference
     */
    pub fn os_user(&self) -> &OSUser {
        &self.m_os_user
    }

    /**
     * Updates the owner `OSUser`
     */
    pub fn set_os_user(&mut self, new_owner_user: OSUser) {
        self.m_os_user = new_owner_user;
    }

    /**
     * Returns the owner `OSGroup` reference
     */
    pub fn os_group(&self) -> &OSGroup {
        &self.m_os_group
    }

    /**
     * Updates the owner `OSGroup`
     */
    pub fn set_os_group(&mut self, new_owner_group: OSGroup) {
        self.m_os_group = new_owner_group;
    }

    /**
     * Returns the protection `Grants` reference
     */
    pub fn protection(&self) -> &Grants<T> {
        &self.m_protection
    }

    /**
     * Returns the mutable protection `Grants` reference
     */
    pub fn protection_mut(&mut self) -> &mut Grants<T> {
        &mut self.m_protection
    }
}